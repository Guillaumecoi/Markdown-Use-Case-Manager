# CLI Commands

> **📝 NOTE**: For the most current and comprehensive CLI documentation, see [CLI Reference Guide](../guides/cli-reference.md).

All the commands you can use with MUCM!

## The Basics

```bash
mucm [command] [options]
```

**Quick shortcuts:**
- `mucm -h` - Show help
- `mucm -i` - Interactive mode (asks you questions)
- `mucm -V` - Show version

## Commands

### `init` - Set Up a New Project

Start a new project with MUCM.

```bash
mucm init [options]
```

**Options:**
- `-m, --methodology <style>` - Pick your style: simple, business, testing
- `-l, --language <lang>` - Test language: rust, python, or none
- `-b, --backend <backend>` - Storage backend: toml (default) or sqlite

**Examples:**
```bash
# Set up with Business methodology
mucm init --methodology business

# Set up with SQLite backend for large projects
mucm init --backend sqlite

# Set up with Python tests and Testing methodology
mucm init --language python --methodology testing

# Combine options
mucm init --methodology business --backend sqlite

# Let MUCM ask you questions (easiest!)
mucm init
```

**Which methodology should I pick?**
- `simple` - Quick and easy, no complicated stuff
- `business` - Good for business analysis and stakeholder focus
- `testing` - Great if you write automated tests and focus on quality

**Which storage backend should I pick?**
- `toml` (default) - Human-readable files, great for small/medium projects (< 100 use cases)
- `sqlite` - Database storage, better performance for large projects (100+ use cases)

**Simple**: Lightweight, rapid development
```bash  
mucm init -m simple
# Creates: Minimal templates, fast documentation
```

**Business**: Enterprise-focused, stakeholder analysis
```bash
mucm init -m business  
# Creates: Business-focused templates, stakeholder emphasis
```

**Testing**: Test-driven development and quality assurance
```bash
mucm init -m testing
# Creates: Test-focused configuration, QA templates
```

### `create` - Create Use Case

Create a new use case with automatic ID generation.

```bash
mucm create <TITLE> [OPTIONS]
```

#### Arguments
- `<TITLE>` - Use case title (required)

#### Options
- `-c, --category <CATEGORY>` - Use case category
- `--priority <PRIORITY>` - Priority level (low, medium, high)
- `--extended` - Include extended metadata fields

#### Examples
```bash
# Basic use case creation
mucm create "User Login"

# With category and priority
mucm create "Process Payment" --category "Finance" --priority high

# With extended metadata
mucm create "Generate Report" --category "Reporting" --extended
```

### `add-scenario` - Add Scenario

Add a scenario to an existing use case.

```bash
mucm add-scenario <USE_CASE_ID> <TITLE> [OPTIONS]
```

#### Arguments
- `<USE_CASE_ID>` - Target use case identifier
- `<TITLE>` - Scenario title

#### Options
- `--priority <PRIORITY>` - Scenario priority (low, medium, high)

#### Examples
```bash
# Add basic scenario
mucm add-scenario UC-SEC-001 "Login with email and password"

# Add high-priority scenario
mucm add-scenario UC-PAY-001 "Process credit card payment" --priority high
```

### `update-status` - Update Status

Update the status of use cases or scenarios.

```bash
mucm update-status <ID> --status <STATUS>
```

#### Arguments
- `<ID>` - Use case or scenario identifier

#### Options
- `--status <STATUS>` - New status (planned, in_progress, implemented, tested, deployed, deprecated)

#### Examples
```bash
# Update use case status
mucm update-status UC-SEC-001 --status implemented

# Update scenario status  
mucm update-status UC-SEC-001-S01 --status tested
```

### `list` - List Use Cases

Display all use cases with their current status.

```bash
mucm list [OPTIONS]
```

#### Options
- `--category <CATEGORY>` - Filter by category
- `--status <STATUS>` - Filter by status
- `--format <FORMAT>` - Output format (table, json, markdown)

#### Examples
```bash
# List all use cases
mucm list

# Filter by category
mucm list --category Security

# JSON output for scripting
mucm list --format json
```

### `status` - Project Status

Show overall project status and progress summary.

```bash
mucm status [OPTIONS]
```

#### Options
- `--detailed` - Show detailed breakdown by category
- `--format <FORMAT>` - Output format (table, json)

#### Examples
```bash
# Basic status overview
mucm status

# Detailed breakdown
mucm status --detailed
```

### `interactive` - Interactive Mode

Launch the interactive terminal interface.

```bash
mucm interactive
# or
mucm -i
```

#### Interactive Features
- **Guided use case creation** with auto-completion
- **Status management** with visual indicators
- **Project overview** with progress tracking
- **Settings configuration** with validation

## Configuration

### Configuration File Location
```
.config/.mucm/mucm.toml
```

### Key Configuration Sections

#### Project Settings
```toml
[project]
name = "My Project"
description = "Project description"
```

#### Directory Configuration
```toml
[directories]
use_case_dir = "docs/use-cases"
test_dir = "tests/use-cases"
```

#### Template Settings
```toml
[templates]
methodology = "business"
use_extended_metadata = true
persona_template_enabled = true
```

#### Generation Options
```toml
[generation]
test_language = "rust"
auto_generate_tests = true
```

## Exit Codes

- `0` - Success
- `1` - General error
- `2` - Configuration error
- `3` - File system error
- `4` - Validation error

## Environment Variables

- `MUCM_CONFIG_DIR` - Override default configuration directory
- `MUCM_LOG_LEVEL` - Set logging level (error, warn, info, debug, trace)

## Common Workflows

### Starting a New Project
```bash
# 1. Initialize with methodology
mucm init --methodology business

# 2. Create first use case
mucm create "User Authentication" --category "Security"

# 3. Add scenarios
mucm add-scenario UC-SEC-001 "Login with valid credentials"
mucm add-scenario UC-SEC-001 "Handle invalid credentials"

# 4. Update status as you progress
mucm update-status UC-SEC-001-S01 --status implemented
```

### Daily Development Workflow
```bash
# Check project status
mucm status

# Work in interactive mode
mucm -i

# Quick status updates
mucm update-status UC-PAY-002 --status tested
```

### Documentation Generation
```bash
# List all use cases for overview
mucm list --format markdown > docs/use-cases-summary.md

# Export project status
mucm status --format json > reports/project-status.json
```

## Tips and Best Practices

### ID Generation
- IDs are automatically generated based on category: `UC-[CATEGORY]-[NUMBER]`
- Scenarios get sequential IDs: `UC-[CATEGORY]-[NUMBER]-S[SCENARIO_NUMBER]`

### Category Naming
- Use PascalCase: `UserManagement`, `DataProcessing`
- Keep categories broad but meaningful
- Consistent naming across your project

### Interactive Mode Benefits
- Auto-completion for existing categories and use cases
- Visual status indicators and progress tracking
- Validation and error prevention
- Guided workflows for complex operations

### Scripting and Automation
- All commands support JSON output for parsing
- Exit codes enable reliable error handling
- Configuration via environment variables
- Batch operations through shell scripts

## Troubleshooting

### Common Issues

**"Configuration file not found"**
```bash
# Solution: Initialize the project first
mucm init
```

**"Use case ID not found"**
```bash
# Solution: List use cases to find correct ID
mucm list
```

**"Permission denied"**
```bash
# Solution: Check directory permissions
ls -la .config/.mucm/
```

### Getting Help
- Use `mucm --help` for command overview
- Use `mucm <command> --help` for specific command help
- Interactive mode provides contextual guidance
- Check [troubleshooting guide](troubleshooting.md) for detailed solutions