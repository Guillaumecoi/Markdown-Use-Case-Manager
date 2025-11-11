// Domain entities - Core business objects

// Private modules
mod metadata;
mod status;
mod use_case;

// Explicit public exports (visible to parent modules)
pub use metadata::Metadata;
pub use status::Status;
pub use use_case::{Priority, UseCase};
