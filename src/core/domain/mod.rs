// Domain layer - Pure business logic, framework agnostic

mod entities;
mod repositories;
mod services;

// Re-exports
pub use entities::{
    Metadata, Priority, ReferenceType, Scenario, ScenarioReference, ScenarioStep, ScenarioType,
    Status, UseCase, UseCaseReference,
};
pub use repositories::UseCaseRepository;
pub use services::{ScenarioService, UseCaseService};
