# CLI Reference

This document provides a comprehensive reference for all Markdown Use Case Manager (mucm) CLI commands.

## Command Overview

```bash
mucm [OPTIONS] <COMMAND>
```

## Global Options

- `-i, --interactive`: Launch interactive mode
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Commands

### `init`

Initialize a new project with templates and configuration.

```bash
mucm init [OPTIONS]
```

**Options:**
- `--language <LANGUAGE>`: Programming language for test generation (rust, python, javascript)
- `--methodology <METHODOLOGY>`: Default methodology (developer, tester, business, feature)
- `--storage <STORAGE>`: Storage backend (toml, sqlite)
- `--finalize`: Skip confirmation prompts

**Examples:**
```bash
mucm init
mucm init --language rust --methodology developer
mucm init --storage sqlite
```

### `create`

Create a new use case.

```bash
mucm create [OPTIONS] <TITLE> --category <CATEGORY>
```

**Arguments:**
- `<TITLE>`: Use case title

**Options:**
- `-c, --category <CATEGORY>`: Use case category
- `-d, --description <DESCRIPTION>`: Use case description
- `-m, --methodology <METHODOLOGY>`: Methodology to use

**Examples:**
```bash
mucm create "User Authentication" --category security
mucm create "Data Export" --category api --description "Export user data in various formats"
```

### `list`

List all use cases in the project.

```bash
mucm list
```

**Examples:**
```bash
mucm list
```

### `status`

Show project status and statistics.

```bash
mucm status
```

**Examples:**
```bash
mucm status
```

### `languages`

List available programming languages for test generation.

```bash
mucm languages
```

**Examples:**
```bash
mucm languages
```

### `methodologies`

List available methodologies.

```bash
mucm methodologies
```

**Examples:**
```bash
mucm methodologies
```

### `methodology-info`

Show detailed information about a specific methodology.

```bash
mucm methodology-info <NAME>
```

**Arguments:**
- `<NAME>`: Methodology name

**Examples:**
```bash
mucm methodology-info developer
```

### `regenerate`

Regenerate markdown files for use cases.

```bash
mucm regenerate [OPTIONS]
```

**Options:**
- `--use-case-id <USE_CASE_ID>`: Specific use case ID to regenerate
- `--methodology <METHODOLOGY>`: Methodology to use for regeneration
- `-a, --all`: Regenerate all use cases

**Examples:**
```bash
mucm regenerate --all
mucm regenerate --use-case-id UC-SEC-001 --methodology developer
```

## Field Management Commands

### Precondition Management

#### `precondition add`

Add a precondition to a use case.

```bash
mucm precondition add <USE_CASE_ID> <PRECONDITION>
```

**Arguments:**
- `<USE_CASE_ID>`: Use case ID (e.g., UC-SEC-001)
- `<PRECONDITION>`: Precondition text

**Examples:**
```bash
mucm precondition add UC-SEC-001 "User must be authenticated"
mucm precondition add UC-API-001 "API key must be valid"
```

#### `precondition list`

List all preconditions for a use case.

```bash
mucm precondition list <USE_CASE_ID>
```

**Arguments:**
- `<USE_CASE_ID>`: Use case ID (e.g., UC-SEC-001)

**Examples:**
```bash
mucm precondition list UC-SEC-001
```

#### `precondition remove`

Remove a precondition from a use case.

```bash
mucm precondition remove <USE_CASE_ID> <INDEX>
```

**Arguments:**
- `<USE_CASE_ID>`: Use case ID (e.g., UC-SEC-001)
- `<INDEX>`: Precondition index (1-based)

**Examples:**
```bash
mucm precondition remove UC-SEC-001 1
```

### Postcondition Management

#### `postcondition add`

Add a postcondition to a use case.

```bash
mucm postcondition add <USE_CASE_ID> <POSTCONDITION>
```

**Arguments:**
- `<USE_CASE_ID>`: Use case ID (e.g., UC-SEC-001)
- `<POSTCONDITION>`: Postcondition text

**Examples:**
```bash
mucm postcondition add UC-SEC-001 "User session is established"
mucm postcondition add UC-API-001 "Response is sent to client"
```

#### `postcondition list`

List all postconditions for a use case.

```bash
mucm postcondition list <USE_CASE_ID>
```

**Arguments:**
- `<USE_CASE_ID>`: Use case ID (e.g., UC-SEC-001)

**Examples:**
```bash
mucm postcondition list UC-SEC-001
```

#### `postcondition remove`

Remove a postcondition from a use case.

```bash
mucm postcondition remove <USE_CASE_ID> <INDEX>
```

**Arguments:**
- `<USE_CASE_ID>`: Use case ID (e.g., UC-SEC-001)
- `<INDEX>`: Postcondition index (1-based)

**Examples:**
```bash
mucm postcondition remove UC-SEC-001 1
```

### Reference Management

#### `reference add`

Add a reference relationship to a use case.

```bash
mucm reference add [OPTIONS] <USE_CASE_ID> <TARGET_ID> <RELATIONSHIP>
```

**Arguments:**
- `<USE_CASE_ID>`: Source use case ID (e.g., UC-SEC-001)
- `<TARGET_ID>`: Target use case ID (e.g., UC-AUTH-001)
- `<RELATIONSHIP>`: Relationship type (dependency, extension, inclusion, alternative)

**Options:**
- `-d, --description <DESCRIPTION>`: Optional description of the relationship

**Examples:**
```bash
mucm reference add UC-SEC-001 UC-AUTH-001 dependency "Requires authentication"
mucm reference add UC-API-001 UC-SEC-001 extension "Extends security features"
```

#### `reference list`

List all references for a use case.

```bash
mucm reference list <USE_CASE_ID>
```

**Arguments:**
- `<USE_CASE_ID>`: Use case ID (e.g., UC-SEC-001)

**Examples:**
```bash
mucm reference list UC-SEC-001
```

#### `reference remove`

Remove a reference from a use case.

```bash
mucm reference remove <USE_CASE_ID> <TARGET_ID>
```

**Arguments:**
- `<USE_CASE_ID>`: Source use case ID (e.g., UC-SEC-001)
- `<TARGET_ID>`: Target use case ID to remove (e.g., UC-AUTH-001)

**Examples:**
```bash
mucm reference remove UC-SEC-001 UC-AUTH-001
```

### `persona`

Manage personas (user archetypes for scenarios).

#### `persona create`

Create a new persona.

```bash
mucm persona create [OPTIONS] <NAME>
```

**Arguments:**
- `<NAME>`: Persona name

**Options:**
- `-d, --description <DESCRIPTION>`: Persona description
- `-g, --goals <GOALS>`: Comma-separated list of goals
- `-t, --tech-level <TECH_LEVEL>`: Technical level (0-10)
- `-c, --context <CONTEXT>`: Usage context

**Examples:**
```bash
mucm persona create "Power User Sarah"
mucm persona create "Developer Dan" --description "Senior backend developer" --tech-level 9
mucm persona create "Business User Bob" --goals "Reduce costs,Improve efficiency" --tech-level 3
```

#### `persona list`

List all personas in the project.

```bash
mucm persona list
```

**Examples:**
```bash
mucm persona list
```

#### `persona show`

Show detailed information about a specific persona.

```bash
mucm persona show <PERSONA_ID>
```

**Arguments:**
- `<PERSONA_ID>`: Persona ID (e.g., power-user-sarah)

**Examples:**
```bash
mucm persona show power-user-sarah
```

#### `persona delete`

Delete a persona.

```bash
mucm persona delete <PERSONA_ID>
```

**Arguments:**
- `<PERSONA_ID>`: Persona ID to delete (e.g., power-user-sarah)

**Examples:**
```bash
mucm persona delete power-user-sarah
```

### `interactive`

Launch interactive mode for guided workflows.

```bash
mucm interactive
```

**Examples:**
```bash
mucm interactive
```

## Relationship Types

When adding references, use one of these relationship types:

- `dependency`: This use case depends on the target use case
- `extension`: This use case extends the functionality of the target use case
- `inclusion`: This use case includes the functionality of the target use case
- `alternative`: This use case provides an alternative to the target use case

## Exit Codes

- `0`: Success
- `1`: Error occurred

## Examples

### Complete Workflow

```bash
# Initialize project
mucm init --language rust --methodology developer

# Create use cases
mucm create "User Authentication" --category security
mucm create "Password Reset" --category security
mucm create "API Access" --category api

# Add preconditions and postconditions
mucm precondition add UC-SEC-001 "User has valid email address"
mucm postcondition add UC-SEC-001 "User is logged in"

# Add relationships
mucm reference add UC-API-001 UC-SEC-001 dependency "Requires authentication"

# View results
mucm list
mucm status
```

### Managing Complex Use Cases

```bash
# Create a complex use case with multiple dependencies
mucm create "Data Export Feature" --category api

# Add multiple preconditions
mucm precondition add UC-API-002 "User has export permissions"
mucm precondition add UC-API-002 "Data exists in system"
mucm precondition add UC-API-002 "Export format is supported"

# Add postconditions
mucm postcondition add UC-API-002 "Export file is generated"
mucm postcondition add UC-API-002 "Download link is provided"

# Link to related use cases
mucm reference add UC-API-002 UC-SEC-001 dependency "Authentication required"
mucm reference add UC-API-002 UC-API-001 extension "Extends API capabilities"
```