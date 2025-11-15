// Domain layer - Pure business logic, framework agnostic

mod entities;
mod repositories;
mod services;

// Re-exports
pub use entities::{
    Actor, Metadata, Persona, Priority, ReferenceType, Scenario, ScenarioReference, ScenarioStep,
    ScenarioType, Status, UseCase, UseCaseReference,
};
pub use repositories::{PersonaRepository, UseCaseRepository};
pub use services::{ScenarioReferenceValidator, UseCaseService};
