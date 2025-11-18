// Domain layer - Pure business logic, framework agnostic

mod entities;
mod repositories;
mod services;

// Re-exports
pub use entities::{
    Actor, Metadata, MethodologyView, Persona, ReferenceType, Scenario, ScenarioReference,
    ScenarioStep, ScenarioType, Status, UseCase, UseCaseReference,
};
pub use repositories::PersonaRepository;
pub use services::{ScenarioReferenceValidator, UseCaseService};
