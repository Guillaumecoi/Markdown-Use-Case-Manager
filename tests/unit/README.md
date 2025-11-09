# Unit Tests Structure

This directory contains comprehensive unit tests for the markdown-use-case-manager (mucm) library. Tests are organized into logical subdirectories by component type for better maintainability.

## Directory Structure

```
tests/unit/
├── cli/                    # Command-line interface unit tests
│   ├── auto_init_test.rs      # Auto-initialization and configuration
│   ├── interactive_test.rs    # Interactive mode functionality
│   ├── methodology_test.rs    # Methodology selection and switching
│   └── runner_test.rs         # CLI runner and command orchestration
│
├── models/                 # Data model unit tests
│   ├── metadata_test.rs       # UseCase metadata (timestamps, versioning)
│   ├── priority_test.rs       # Priority enum and parsing
│   ├── scenario_test.rs       # Scenario model and status updates
│   ├── status_test.rs         # Status enum and transitions
│   └── use_case_test.rs       # UseCase model and aggregations
│
├── core/                   # Core functionality unit tests
│   ├── config_test.rs                    # Configuration management
│   ├── coordinator_methodology_test.rs   # Methodology coordination
│   ├── language_test.rs                  # Language support system
│   ├── processor_test.rs                 # Processor integration
│   └── template_engine_test.rs           # Template engine and rendering
│
├── services/               # Service layer unit tests
│   └── use_case_service_test.rs # UseCase business logic and ID generation
│
├── features/               # Extended feature unit tests
│   └── persona_test.rs        # Persona management
│
└── mod.rs                  # Module organization
```

## Test Categories

### CLI Tests (`cli/`)
Unit tests for command-line interface components:
- Auto-initialization detection and setup
- Interactive mode prompts and flows
- Methodology selection and validation
- CLI runner coordination and command execution

**Test Count**: ~20 tests

### Model Tests (`models/`)
Unit tests for core data models:
- **UseCase**: Creation, status aggregation, scenario management
- **Scenario**: Creation, status updates, serialization
- **Status**: Enum parsing, transitions, display
- **Priority**: Enum parsing, ordering, defaults
- **Metadata**: Timestamps, versioning, serialization

**Test Count**: ~40 tests

### Core Tests (`core/`)
Unit tests for core system functionality:
- **Configuration**: Loading, saving, validation, directory settings
- **Coordinator**: Methodology management, use case orchestration
- **Language Support**: Registry, lookup, template integration
- **Processors**: Methodology processors and integration
- **Template Engine**: Rendering, language templates, error handling

**Test Count**: ~25 tests

### Service Tests (`services/`)
Unit tests for service layer:
- UseCase service business logic
- Unique ID generation
- Category management
- File system abstraction

**Test Count**: ~5 tests

### Feature Tests (`features/`)
Unit tests for extended features:
- Persona creation and management
- Persona directory configuration
- Extended metadata handling

**Test Count**: ~10 tests

## Running Tests

```bash
# Run all unit tests
cargo nextest run --lib --all-features

# Run specific test module
cargo nextest run --lib --all-features -E 'test(cli::)'
cargo nextest run --lib --all-features -E 'test(models::)'
cargo nextest run --lib --all-features -E 'test(core::)'
cargo nextest run --lib --all-features -E 'test(services::)'
cargo nextest run --lib --all-features -E 'test(features::)'

# Run specific test file
cargo nextest run --lib --all-features -E 'test(models::use_case_test::)'
cargo nextest run --lib --all-features -E 'test(cli::runner_test::)'

# Run individual test
cargo test --lib test_scenario_new
```

## Test Guidelines

1. **Pure Unit Tests**: Tests focus on single units/functions in isolation
2. **No I/O**: Use mocks and in-memory structures where possible
3. **Fast Execution**: Unit tests should be extremely fast (< 1ms each)
4. **Naming Convention**: `test_<component>_<behavior>` (e.g., `test_scenario_set_status`)
5. **Arrange-Act-Assert**: Follow AAA pattern for clarity
6. **Documentation**: Each test should be self-documenting with clear assertions

## Test Structure Example

```rust
#[test]
fn test_scenario_set_status() {
    // Arrange: Set up test data
    let scenario = Scenario::new(
        "S01".to_string(),
        "Test Scenario".to_string(),
        Some("Description".to_string()),
    );

    // Act: Perform the operation
    let updated = scenario.set_status(Status::InProgress);

    // Assert: Verify the results
    assert_eq!(updated.status, Status::InProgress);
    assert!(updated.metadata.updated_at > scenario.metadata.updated_at);
}
```

## Comparison with Integration Tests

| Aspect | Unit Tests | Integration Tests |
|--------|------------|-------------------|
| **Scope** | Single function/struct | Full workflows |
| **I/O** | Mocked/in-memory | Real file system |
| **Speed** | < 1ms per test | 10-500ms per test |
| **Dependencies** | Isolated | Multiple components |
| **Purpose** | Verify logic | Verify integration |

## Recent Changes

- **2025-10-20**: Reorganized tests into subdirectories (`cli/`, `models/`, `core/`, `services/`, `features/`)
- **2025-10-20**: Renamed files to remove redundant prefixes (e.g., `modular_language_test.rs` → `language_test.rs`)
- **2025-10-20**: Created clear separation between models, core, and service tests
- **2025-10-20**: Added comprehensive README documentation

## Total Test Count

**28 unit tests** (part of the total 309 tests: 28 unit + 112 integration + 169 template tests)

## Related Documentation

- **Integration Tests**: See `tests/integration/README.md` for full workflow tests
- **Test Utils**: See `tests/test_utils.rs` for shared test utilities
