/// Godseed Core — Root library module
///
/// Architecture: headless Bevy ECS simulation.
/// The player entity participates in the same systems as NPCs.
/// Terminal UI layer writes to PlayerInputBuffer; simulation drains it each tick.

pub mod types;
pub mod components;
pub mod resources;
pub mod world;
pub mod content;
pub mod settlement;
pub mod household;
pub mod player;
pub mod npc;
pub mod events;
pub mod persistence;
pub mod replay;
pub mod invariants;
pub mod telemetry;
pub mod systems;
pub mod sim;

// Re-export primary simulation entrypoint
pub use sim::Simulation;
pub use types::*;
