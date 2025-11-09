// Domain entities - Core business objects

pub mod metadata;
pub mod status;
pub mod use_case;

// Re-export commonly used types
pub use metadata::Metadata;
pub use status::Status;
pub use use_case::{Priority, UseCase};
