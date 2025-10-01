// Integration test runner - imports and runs all integration tests
mod integration;

// Re-export all integration test modules so they get discovered by cargo test
pub use integration::*;