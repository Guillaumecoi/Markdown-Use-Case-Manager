// Unit tests module organization
//
// Most unit tests have been moved to the implementation files themselves
// using #[cfg(test)] mod tests { ... } blocks, following Rust best practices.
//
// Remaining tests here are for higher-level integration scenarios:
// - cli/: Command-line interface unit tests
// - features/: Extended feature tests

// CLI unit tests
pub mod cli;

// Extended functionality unit tests
pub mod features;
