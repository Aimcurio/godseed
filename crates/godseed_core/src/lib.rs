#![allow(
    clippy::collapsible_match,
    clippy::derivable_impls,
    clippy::io_other_error,
    clippy::let_and_return,
    clippy::manual_clamp,
    clippy::manual_is_multiple_of,
    clippy::manual_range_contains,
    clippy::match_like_matches_macro,
    clippy::new_without_default,
    clippy::should_implement_trait,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::unwrap_or_default,
    clippy::useless_format
)]

pub mod components;
pub mod content;
pub mod events;
pub mod household;
pub mod invariants;
pub mod npc;
pub mod persistence;
pub mod player;
pub mod replay;
pub mod resources;
pub mod settlement;
pub mod sim;
pub mod systems;
pub mod telemetry;
/// Godseed Core — Root library module
///
/// Architecture: headless Bevy ECS simulation.
/// The player entity participates in the same systems as NPCs.
/// Terminal UI layer writes to PlayerInputBuffer; simulation drains it each tick.
pub mod types;
pub mod world;

// Re-export primary simulation entrypoint
pub use sim::Simulation;
pub use types::*;
