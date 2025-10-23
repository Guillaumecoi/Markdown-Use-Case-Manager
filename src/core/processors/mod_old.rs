// src/core/processors/mod.rs

//! Modular processor system for handling different use case methodologies
//! 
//! This module provides a unified, template-driven approach to methodology processing
//! with enhanced extensibility and configuration capabilities.

// Core types and traits
pub mod methodology_processor;
pub mod methodology_manager;

// Unified processor system components
pub mod unified_processor;
pub mod unified_registry;

// Utility modules for enhanced modularity
pub mod factory;
pub mod config;

// Template-driven processor system - the future!
pub mod template_driven;

// Re-exports for public API

/// Core processor traits and types
pub use methodology_processor::{
    MethodologyProcessor, 
    ProcessedScenarios, 
    UseCaseContext,
};

/// Unified system - primary recommended API
pub mod unified {
    //! Modern unified processor system
    //! 
    //! This system provides a flexible, configuration-driven approach to 
    //! methodology processing that's easier to extend and customize.
    
    pub use super::unified_processor::{
        UnifiedProcessor, 
        MethodologyConfig, 
        ProcessorFunction
    };
    pub use super::unified_registry::{
        RegistryBuilder
    };
    
    /// Factory functions for creating processors
    pub use super::factory::{
        ProcessorFactory,
        MethodologyConfigBuilder
    };
    
    /// Configuration management
    pub use super::config::{
        ConfigManager,
        ProcessorSetConfig,
        SerializableMethodologyConfig,
        ConfigError
    };
}

// Convenience re-exports for most common usage
pub use methodology_manager::MethodologyManager;

// Unified system - primary recommended API
pub use unified_processor::{UnifiedProcessor, MethodologyConfig, ProcessorFunction};
pub use unified_registry::RegistryBuilder;

// Factory convenience exports
pub use factory::{ProcessorFactory, MethodologyConfigBuilder};

// Config convenience exports
pub use config::{ConfigManager, ProcessorSetConfig};

// Template-driven processor exports
pub use template_driven::TemplateProcessor;