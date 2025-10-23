// src/core/processors/mod.rs
pub mod methodology_processor;
pub mod methodology_manager;
pub mod methodologies;
pub mod processor_registry;
pub mod unified_processor;
pub mod unified_registry;

pub use methodology_processor::{MethodologyProcessor, ProcessedScenarios, UseCaseContext};
pub use methodology_manager::MethodologyManager;
pub use processor_registry::create_default_registry;

// Unified system exports - ready for future integration
#[allow(unused_imports)]
pub use unified_processor::{UnifiedProcessor, MethodologyConfig, ProcessorFunction};
#[allow(unused_imports)]
pub use unified_registry::{create_unified_registry, RegistryBuilder};