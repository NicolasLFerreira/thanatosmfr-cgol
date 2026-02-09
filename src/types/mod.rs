//! The `types` module houses common types used between different modules.

mod canonical_configuration;
mod cell_configuration;
mod cell_coord;
mod mfrac_report;
mod simulation_feed;
mod simulation_payload;
mod simulation_state;

// Re-exports for a flattened API
pub use canonical_configuration::CanonicalConfiguration;
pub use cell_configuration::CellConfiguration;
pub use cell_coord::CellCoord;
pub use mfrac_report::*;
pub use simulation_feed::SimulationFeed;
pub use simulation_payload::SimulationPayload;
pub use simulation_state::SimulationState;
