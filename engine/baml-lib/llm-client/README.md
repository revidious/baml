# BAML LLM Client

> **⚠️ IMPORTANT NOTE**
>
> This document was initially generated by an AI assistant and should be taken with a grain of salt. While it provides a good starting point, some information might be inaccurate or outdated. We encourage contributors to manually update this document and remove this note once the content has been verified and corrected by the team.
>
> If you find any inaccuracies or have improvements to suggest, please feel free to submit a PR updating this guide.

The LLM client manages integrations with various LLM providers, handling authentication, request configuration, and response processing. It provides a unified interface for interacting with different LLM services.

## Supported Providers

```rust
pub enum ClientProvider {
    OpenAI(OpenAIClientProviderVariant), // Base, Azure, Custom
    Anthropic,                           // Claude models
    AwsBedrock,                          // AWS hosted models
    GoogleAi,                            // Gemini models
    Vertex,                              // PaLM models
    Strategy(StrategyClientProvider),    // Load balancing
}
```

## Authentication

```rust
pub struct ServiceAccount {
    pub token_uri: String,
    pub project_id: String,
    pub client_email: String,
    pub private_key: String,
}

pub enum AuthMethod {
    ApiKey(String),
    ServiceAccount(ServiceAccount),
    Custom(Box<dyn AuthProvider>),
}
```

## Load Balancing & Resilience

### Strategies
- Round-robin: Distribute requests across providers
- Fallback: Automatic retry with alternative providers
- Custom role management for different model capabilities

```baml
// Example configuration
client<llm> Resilient_LLM {
  provider baml-fallback
  options {
    strategy [
      GPT4
      Claude
    ]
  }
}
```

### Retry Policies
```baml
retry_policy Resilient {
  max_retries 3
  strategy {
    type exponential_backoff
    initial_delay 1000
    max_delay 30000
  }
}
```

## Development

### Adding a New Provider

1. Create provider module in `clients/`:
   ```rust
   pub struct NewProvider {
       base_url: StringOr,              // API endpoint
       api_key: StringOr,               // Authentication
       model: Option<StringOr>,         // Model selection
       headers: IndexMap<String, StringOr>, // Custom headers
       properties: IndexMap<String, Value>, // Model parameters
   }
   ```

2. Implement provider traits:
   ```rust
   impl ClientProvider {
       fn create_from(&self, properties: PropertyHandler) -> Result<Self> {
           let api_key = properties.ensure_api_key();
           let model = properties.ensure_string("model", true);
           // Provider-specific setup
       }
   }
   ```

3. Add to `ClientProvider` enum and update tests

### Testing

```bash
# Test all providers
cargo test -p internal-llm-client

# Test specific provider
cargo test -p internal-llm-client openai
cargo test -p internal-llm-client anthropic
```

## Examples

### Basic Provider Configuration
```baml
client<llm> GPT4 {
  provider openai
  retry_policy Resilient
  options {
    model gpt-4
    api_key env.OPENAI_API_KEY
    max_tokens 1000
  }
}
```

### Azure OpenAI Configuration
```baml
client<llm> AzureGPT {
  provider openai/azure
  options {
    model deployment_name
    api_key env.AZURE_API_KEY
    endpoint env.AZURE_ENDPOINT
  }
}
```

### AWS Bedrock Configuration
```baml
client<llm> Claude3 {
  provider aws-bedrock
  options {
    model anthropic.claude-3-sonnet
    region us-west-2
    credentials {
      access_key_id env.AWS_ACCESS_KEY_ID
      secret_access_key env.AWS_SECRET_ACCESS_KEY
    }
  }
}
```

### Load Balancing Example
```baml
client<llm> SmartLLM {
  provider baml-strategy
  options {
    strategy round_robin
    providers [
      {
        client GPT4
        weight 2
      }
      {
        client Claude
        weight 1
      }
    ]
  }
}
```

## Error Handling

```rust
#[derive(Debug, Error)]
pub enum LLMError {
    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Provider error: {0}")]
    ProviderError(String),
}

impl LLMClient {
    pub fn handle_error(&self, error: LLMError) -> Result<()> {
        match error {
            LLMError::RateLimit(_) => self.retry_with_backoff(),
            LLMError::ProviderError(_) => self.try_fallback_provider(),
            _ => Err(error),
        }
    }
}
```

## Streaming Support

```rust
impl LLMClient {
    pub async fn stream_completion(&self, request: CompletionRequest) -> Result<impl Stream<Item = CompletionChunk>> {
        // Provider-specific streaming implementation
    }
}

// Usage example
let stream = client.stream_completion(request).await?;
tokio::pin!(stream);
while let Some(chunk) = stream.next().await {
    println!("Received chunk: {}", chunk);
}
```