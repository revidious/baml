use anyhow::Result;
use std::sync::{mpsc, Arc, Mutex};
use tokio::sync::watch;
use web_time::{Duration, Instant};

use crate::{
    on_log_event::{LogEvent, LogEventCallbackSync, LogEventMetadata},
    tracing::api_wrapper::core_types::{ContentPart, MetadataType, Template, ValueType},
    TraceStats,
};

use super::api_wrapper::{core_types::LogSchema, APIConfig, APIWrapper, BoundaryAPI};

const MAX_TRACE_SEND_CONCURRENCY: usize = 10;

enum TxEventSignal {
    #[allow(dead_code)]
    Stop,
    Flush(u128),
    Submit(LogSchema),
}

enum ProcessorStatus {
    Active,
    Done(u128),
}

struct DeliveryThread {
    api_config: Arc<APIWrapper>,
    span_rx: mpsc::Receiver<TxEventSignal>,
    stop_tx: watch::Sender<ProcessorStatus>,
    rt: tokio::runtime::Runtime,
    max_batch_size: usize,
    max_concurrency: Arc<tokio::sync::Semaphore>,
    stats: TraceStats,
}

impl DeliveryThread {
    fn new(
        api_config: APIWrapper,
        span_rx: mpsc::Receiver<TxEventSignal>,
        stop_tx: watch::Sender<ProcessorStatus>,
        max_batch_size: usize,
        stats: TraceStats,
    ) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();

        Self {
            api_config: Arc::new(api_config),
            span_rx,
            stop_tx,
            rt,
            max_batch_size,
            max_concurrency: tokio::sync::Semaphore::new(MAX_TRACE_SEND_CONCURRENCY).into(),
            stats,
        }
    }

    async fn process_batch(&self, batch: Vec<LogSchema>) {
        let work = batch
            .into_iter()
            .map(|work| {
                let api_config = self.api_config.clone();
                let semaphore = self.max_concurrency.clone();
                let stats = self.stats.clone();
                stats.guard().send();

                let stats_clone = stats.clone();
                async move {
                    let guard = stats_clone.guard();
                    let Ok(_acquired) = semaphore.acquire().await else {
                        log::warn!(
                            "Failed to acquire semaphore because it was closed - not sending span"
                        );
                        return;
                    };
                    match api_config.log_schema(&work).await {
                        Ok(_) => {
                            guard.done();
                            log::debug!(
                                "Successfully sent log schema: {} - {:?}",
                                work.event_id,
                                work.context.event_chain.last()
                            );
                        }
                        Err(e) => {
                            log::warn!("Unable to emit BAML logs: {:#?}", e);
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        // Wait for all the futures to complete
        futures::future::join_all(work).await;
    }

    fn run(&self) {
        let mut batch = Vec::with_capacity(self.max_batch_size);
        let mut now = Instant::now();
        loop {
            // Try to fill the batch up to max_batch_size
            let (batch_full, flush, exit) =
                match self.span_rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(TxEventSignal::Submit(work)) => {
                        self.stats.guard().submit();
                        batch.push(work);
                        (batch.len() >= self.max_batch_size, None, false)
                    }
                    Ok(TxEventSignal::Flush(id)) => (false, Some(id), false),
                    Ok(TxEventSignal::Stop) => (false, None, true),
                    Err(mpsc::RecvTimeoutError::Timeout) => (false, None, false),
                    Err(mpsc::RecvTimeoutError::Disconnected) => (false, None, true),
                };

            let time_trigger = now.elapsed().as_millis() >= 1000;

            let should_process_batch =
                (batch_full || flush.is_some() || exit || time_trigger) && !batch.is_empty();

            // Send events every 1 second or when the batch is full
            if should_process_batch {
                self.rt
                    .block_on(self.process_batch(std::mem::take(&mut batch)));
            }

            if should_process_batch || time_trigger {
                now = std::time::Instant::now();
            }

            if let Some(id) = flush {
                match self.stop_tx.send(ProcessorStatus::Done(id)) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Error sending flush signal: {:?}", e);
                    }
                }
            }
            if exit {
                return;
            }
        }
    }
}

pub(super) struct ThreadedTracer {
    api_config: Arc<APIWrapper>,
    span_tx: mpsc::Sender<TxEventSignal>,
    stop_rx: watch::Receiver<ProcessorStatus>,
    #[allow(dead_code)]
    join_handle: std::thread::JoinHandle<()>,
    log_event_callback: Arc<Mutex<Option<LogEventCallbackSync>>>,
    stats: TraceStats,
}

impl ThreadedTracer {
    fn start_worker(
        api_config: APIWrapper,
        max_batch_size: usize,
        stats: TraceStats,
    ) -> (
        mpsc::Sender<TxEventSignal>,
        watch::Receiver<ProcessorStatus>,
        std::thread::JoinHandle<()>,
    ) {
        let (span_tx, span_rx) = mpsc::channel();
        let (stop_tx, stop_rx) = watch::channel(ProcessorStatus::Active);
        let join_handle = std::thread::spawn(move || {
            DeliveryThread::new(api_config, span_rx, stop_tx, max_batch_size, stats).run();
        });

        (span_tx, stop_rx, join_handle)
    }

    pub fn new(api_config: &APIWrapper, max_batch_size: usize, stats: TraceStats) -> Self {
        let (span_tx, stop_rx, join_handle) =
            Self::start_worker(api_config.clone(), max_batch_size, stats.clone());

        Self {
            api_config: Arc::new(api_config.clone()),
            span_tx,
            stop_rx,
            join_handle,
            log_event_callback: Arc::new(Mutex::new(None)),
            stats,
        }
    }

    pub fn flush(&self) -> Result<()> {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        self.span_tx.send(TxEventSignal::Flush(id))?;

        let flush_start = Instant::now();

        while flush_start.elapsed() < Duration::from_secs(60) {
            {
                match *self.stop_rx.borrow() {
                    ProcessorStatus::Active => {}
                    ProcessorStatus::Done(r_id) if r_id >= id => {
                        return Ok(());
                    }
                    ProcessorStatus::Done(id) => {
                        // Old flush, ignore
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        anyhow::bail!("BatchProcessor worker thread did not finish in time")
    }

    pub fn set_log_event_callback(&self, log_event_callback: Option<LogEventCallbackSync>) {
        // Get a mutable lock on the log_event_callback
        let mut callback_lock = self.log_event_callback.lock().unwrap();

        *callback_lock = log_event_callback;
    }

    pub fn submit(&self, mut event: LogSchema) -> Result<()> {
        let callback = self.log_event_callback.lock().unwrap();
        if let Some(ref callback) = *callback {
            let event = event.clone();
            let llm_output_model = event.metadata.as_ref().and_then(|m| match m {
                MetadataType::Single(llm_event) => Some(llm_event),
                // take the last element in the vector
                MetadataType::Multi(llm_events) => llm_events.last(),
            });

            let log_event_result = callback(LogEvent {
                metadata: LogEventMetadata {
                    event_id: event.event_id.clone(),
                    parent_id: event.parent_event_id.clone(),
                    root_event_id: event.root_event_id.clone(),
                },
                prompt: llm_output_model.and_then(|llm_event| {
                    match llm_event.clone().input.prompt.template {
                        Template::Single(text) => Some(text),
                        Template::Multiple(chat_prompt) => {
                            serde_json::to_string_pretty(&chat_prompt).ok().or_else(|| {
                                log::debug!(
                                    "Failed to serialize chat prompt for event {}",
                                    event.event_id
                                );
                                None
                            })
                        }
                    }
                }),
                raw_output: llm_output_model
                    .and_then(|llm_event| llm_event.clone().output.map(|output| output.raw_text)),
                parsed_output: event.io.output.and_then(|output| match output.value {
                    // so the string value looks something like:
                    // '"[\"d\", \"e\", \"f\"]"'
                    // so we need to unescape it once and turn it into a normal json
                    // and then stringify it to get:
                    // '["d", "e", "f"]'
                    ValueType::String(value) => serde_json::from_str::<serde_json::Value>(&value)
                        .ok()
                        .and_then(|json_value| json_value.as_str().map(|s| s.to_string()))
                        .or(Some(value)),
                    _ => serde_json::to_string_pretty(&output.value)
                        .ok()
                        .or_else(|| {
                            log::debug!(
                                "Failed to serialize output value for event {}",
                                event.event_id
                            );
                            None
                        }),
                }),
                start_time: event.context.start_time,
            });

            if log_event_result.is_err() {
                log::error!(
                    "Error calling log_event_callback for event id: {}",
                    event.event_id
                );
            }

            log_event_result?;
        }

        // TODO: do the redaction

        // Redact the event
        event = redact_event(event, &self.api_config.config);

        self.span_tx.send(TxEventSignal::Submit(event))?;
        Ok(())
    }
}

fn redact_event(mut event: LogSchema, api_config: &APIConfig) -> LogSchema {
    let redaction_enabled = api_config.log_redaction_enabled();
    let placeholder = api_config.log_redaction_placeholder();

    if !redaction_enabled {
        return event;
    }

    let placeholder = placeholder
        .replace("{root_event.id}", &event.root_event_id)
        .replace("{event.id}", &event.event_id);

    // Redact LLMOutputModel raw_text
    if let Some(metadata) = &mut event.metadata {
        match metadata {
            MetadataType::Single(llm_event) => {
                if let Some(output) = &mut llm_event.output {
                    output.raw_text = placeholder.clone();
                }
            }
            MetadataType::Multi(llm_events) => {
                for llm_event in llm_events {
                    if let Some(output) = &mut llm_event.output {
                        output.raw_text = placeholder.clone();
                    }
                }
            }
        }
    }

    // Redact input IO
    if let Some(input) = &mut event.io.input {
        match &mut input.value {
            ValueType::String(s) => *s = placeholder.clone(),
            ValueType::List(v) => v.iter_mut().for_each(|s| *s = placeholder.clone()),
        }
    }

    // Redact output IO
    if let Some(output) = &mut event.io.output {
        match &mut output.value {
            ValueType::String(s) => *s = placeholder.clone(),
            ValueType::List(v) => v.iter_mut().for_each(|s| *s = placeholder.clone()),
        }
    }

    // Redact LLMEventInput Template
    if let Some(metadata) = &mut event.metadata {
        match metadata {
            MetadataType::Single(llm_event) => {
                redact_template(&mut llm_event.input.prompt.template, &placeholder);
            }
            MetadataType::Multi(llm_events) => {
                for llm_event in llm_events {
                    redact_template(&mut llm_event.input.prompt.template, &placeholder);
                }
            }
        }
    }

    event
}

fn redact_template(template: &mut Template, placeholder: &str) {
    match template {
        Template::Single(s) => *s = placeholder.to_string(),
        Template::Multiple(chats) => {
            for chat in chats {
                for part in &mut chat.content {
                    if let ContentPart::Text(s) = part {
                        *s = placeholder.to_string();
                    }
                }
            }
        }
    }
}
