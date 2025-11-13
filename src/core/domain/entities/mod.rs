// Domain entities - Core business objects

// Private modules
mod metadata;
mod reference_type;
mod status;
mod use_case;

// Explicit public exports (visible to parent modules)
pub use metadata::Metadata;
pub use reference_type::ReferenceType;
pub use status::Status;
pub use use_case::UseCase;
