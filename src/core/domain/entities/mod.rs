// Domain entities - Core business objects

// Private modules
mod actor;
mod metadata;
mod persona;
mod reference_type;
mod scenario;
mod scenario_reference;
mod scenario_step;
mod scenario_type;
mod status;
mod use_case;
mod use_case_reference;

// Explicit public exports (visible to parent modules)
pub use actor::Actor;
pub use metadata::Metadata;
pub use persona::Persona;
pub use reference_type::ReferenceType;
pub use scenario::Scenario;
pub use scenario_reference::ScenarioReference;
pub use scenario_step::ScenarioStep;
pub use scenario_type::ScenarioType;
pub use status::Status;
pub use use_case::UseCase;
pub use use_case_reference::UseCaseReference;
