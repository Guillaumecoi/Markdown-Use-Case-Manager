// src/core/processors/methodologies/mod.rs

pub mod simple_processor;
pub mod business_processor;
pub mod testing_processor;

pub use simple_processor::SimpleProcessor;
pub use business_processor::BusinessProcessor;
pub use testing_processor::TestingProcessor;