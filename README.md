# Use Case Manager

A comprehensive CLI tool and Rust library for managing use cases, scenarios, and documentation with automatic status tracking and test generation.

## Features

- 📋 **Use Case Management**: Create, organize, and track use cases with hierarchical scenarios
- 🎯 **Smart Status Tracking**: Automatic status aggregation from scenarios to use cases
- 📝 **Template-Driven Generation**: Generate documentation and test files from customizable templates
- 🔄 **Multi-Language Support**: Generate tests for Rust (more languages coming soon)
- 📊 **Progress Tracking**: Visual progress indicators and status reporting
- ⚙️ **Configurable**: Project-specific configuration and custom templates

## Quick Start

### Installation

```bash
# Build from source
git clone https://github.com/GuillaumeCoi/use-case-manager
cd use-case-manager
cargo install --path .
```

### Basic Usage

```bash
# Initialize a new project
ucm init

# Create a use case
ucm create "User Authentication" --category "Security" --description "Handle user login and logout"

# List all use cases
ucm list

# Show project status
ucm status
```

## Status Hierarchy

```
PLANNED (📋)     → Basic idea documented
IN_PROGRESS (🔄) → Development started
IMPLEMENTED (⚡) → Code complete, not tested
TESTED (✅)      → Tested and verified
DEPLOYED (🚀)    → Live in production
DEPRECATED (⚠️) → No longer maintained
```

Use case status is automatically computed as the minimum status of all scenarios.

## Configuration

The tool creates a `.ucm/config.toml` file in your project root:

```toml
[project]
name = "My Project"
description = "A project managed with use case manager"

[directories]
use_case_dir = "docs/use-cases"
test_dir = "tests/use-cases"

[generation]
test_language = "rust"
auto_generate_tests = true
```

## Library Usage

```rust
use use_case_manager::{UseCaseManager, UseCase, Status, Priority};

fn main() -> anyhow::Result<()> {
    let mut manager = UseCaseManager::load()?;
    
    let use_case_id = manager.create_use_case(
        "User Registration".to_string(),
        "Authentication".to_string(),
        Some("User provides valid information".to_string())
    )?;
    
    println!("Created use case: {}", use_case_id);
    
    Ok(())
}
```

## Generated Files

### Use Case Documentation
- **Markdown documentation**: `docs/use-cases/UC-XXX-001.md` (with YAML frontmatter)

### Test Files
- **Rust tests**: `tests/use-cases/uc_xxx_001.rs`

## Roadmap

- [ ] Add scenario management commands
- [ ] Support for more test languages (JavaScript, Python)
- [ ] Custom template support
- [ ] Git integration for change tracking
- [ ] Interactive CLI mode
- [ ] Export to various formats (PDF, HTML)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.