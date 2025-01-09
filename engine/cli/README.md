# BAML CLI

> **⚠️ IMPORTANT NOTE**
>
> This document was initially generated by an AI assistant and should be taken with a grain of salt. While it provides a good starting point, some information might be inaccurate or outdated. We encourage contributors to manually update this document and remove this note once the content has been verified and corrected by the team.
>
> If you find any inaccuracies or have improvements to suggest, please feel free to submit a PR updating this guide.

The BAML CLI is the command-line interface for BAML development, providing tools for code generation, validation, and project management.

## Features

- Project initialization
- Code generation
- Type checking
- Schema validation
- Project compilation
- Development server
- Language client generation
- Error reporting

## Installation

```bash
# Using cargo
cargo install baml-cli

# From source
git clone https://github.com/boundaryml/baml
cd baml/engine/cli
cargo install --path .
```

## Usage

### Project Initialization

```bash
# Create new project
baml init my-project

# Initialize in existing directory
cd my-project
baml init
```

### Code Generation

```bash
# Generate all clients
baml generate

# Generate specific language
baml generate python
baml generate typescript
```

### Development

```bash
# Start development server
baml dev

# Watch for changes
baml dev --watch
```

### Validation

```bash
# Validate BAML files
baml check

# Type check
baml check --types
```

## Project Structure

```
cli/
├── src/
│   ├── commands/         # CLI commands
│   │   ├── init.rs       # Project initialization
│   │   ├── generate.rs   # Code generation
│   │   └── dev.rs        # Development server
│   ├── config/           # Configuration handling
│   ├── validation/       # Schema validation
│   └── utils/            # Shared utilities
├── templates/            # Project templates
└── examples/            # Example projects
```

## Development

### Prerequisites

- Rust toolchain
- Cargo
- Project dependencies

### Setup

```bash
# Install dependencies
cargo build

# Run tests
cargo test

# Run CLI
cargo run -- <command>
```

## Adding Features

### 1. New Command

1. Create command module:
```rust
// src/commands/new_command.rs

use clap::Parser;
use crate::command::{Command, CommandResult};

#[derive(Parser)]
pub struct NewCommand {
    #[clap(long, short)]
    option: String,
}

impl Command for NewCommand {
    fn run(&self) -> CommandResult {
        // Implementation
    }
}
```

2. Register command:
```rust
// src/main.rs

cli.add_command("new-command", NewCommand::new());
```

### 2. New Template

1. Add template files:
```
templates/
└── new-template/
    ├── baml.toml
    ├── clients.baml
    └── generators.baml
```

2. Register template:
```rust
// src/templates/mod.rs

impl TemplateManager {
    pub fn register_template(&mut self, name: &str, path: &Path) {
        self.templates.insert(name.to_string(), path.to_path_buf());
    }
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let cmd = NewCommand::new();
        let result = cmd.run();
        assert!(result.is_ok());
    }
}
```

### Integration Tests

```rust
#[test]
fn test_project_generation() {
    let temp_dir = TempDir::new().unwrap();
    let result = Command::new("baml")
        .arg("init")
        .arg("test-project")
        .current_dir(&temp_dir)
        .output()
        .unwrap();
    assert!(result.status.success());
}
```

## Configuration

### Project Config

```toml
# baml.toml

[project]
name = "my-project"
version = "0.1.0"

[generation]
output = "./generated"
languages = ["python", "typescript"]

[providers]
openai = { api_key = "${OPENAI_API_KEY}" }
anthropic = { api_key = "${ANTHROPIC_API_KEY}" }
```

### CLI Config

```toml
# ~/.baml/config.toml

[default]
api_key = "your-api-key"
output_dir = "./baml_generated"

[profiles.dev]
api_key = "dev-api-key"
```

## Error Handling

```rust
#[derive(Debug, Error)]
pub enum CliError {
    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("Generation failed: {0}")]
    Generation(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl Command {
    fn handle_error(&self, error: CliError) -> ExitCode {
        match error {
            CliError::Config(msg) => {
                eprintln!("Configuration error: {}", msg);
                ExitCode::CONFIG_ERROR
            }
            CliError::Generation(msg) => {
                eprintln!("Generation failed: {}", msg);
                ExitCode::GENERATION_ERROR
            }
            CliError::Validation(msg) => {
                eprintln!("Validation error: {}", msg);
                ExitCode::VALIDATION_ERROR
            }
        }
    }
}
```

## Best Practices

1. **Command Structure**
   - Use subcommands
   - Provide help text
   - Support --version

2. **Error Handling**
   - Use custom errors
   - Provide clear messages
   - Set exit codes

3. **Configuration**
   - Use config files
   - Support env vars
   - Validate input

4. **Testing**
   - Test commands
   - Mock file system
   - Check output

## Contributing

1. Read [Contributing Guide](../../CONTRIBUTING.md)
2. Follow CLI guidelines
3. Add tests for new features
4. Update documentation
5. Submit PR for review