// src/core/processors/mod.rs
pub mod methodology_processor;
pub mod methodologies;
pub mod processor_registry;
pub mod unified_processor;
pub mod unified_registry;

pub use methodology_processor::{MethodologyProcessor, ProcessedScenarios, UseCaseContext};
pub use processor_registry::create_default_registry;

// Legacy processors - kept for backward compatibility
#[allow(unused_imports)]
pub use methodologies::{SimpleProcessor, BusinessProcessor, TestingProcessor};

// Unified system exports - ready for future integration
#[allow(unused_imports)]
pub use unified_processor::{UnifiedProcessor, MethodologyConfig, ProcessorFunction};
#[allow(unused_imports)]
pub use unified_registry::{create_unified_registry, RegistryBuilder};