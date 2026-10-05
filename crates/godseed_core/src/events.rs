use crate::telemetry::TelemetryEventType;
use crate::types::{CitizenId, OccupationType, ResourceType};
/// Godseed — Simulation Events
use serde::{Deserialize, Serialize};

// ── Simulation Events (stored in EventRing) ───────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimEvent {
    Birth {
        citizen: CitizenId,
        tick: u64,
    },
    Death {
        citizen: CitizenId,
        cause: DeathCause,
        tick: u64,
    },
    JobChange {
        citizen: CitizenId,
        old: OccupationType,
        new: OccupationType,
        tick: u64,
    },
    PriceChange {
        resource: ResourceType,
        old_price: f32,
        new_price: f32,
        tick: u64,
    },
    PlayerAction {
        tick: u64,
        action_name: String,
        success: bool,
    },
    RelationshipEvent {
        actor: CitizenId,
        target: CitizenId,
        delta: i16,
        tick: u64,
    },
    CapabilityAcquired {
        citizen: CitizenId,
        capability_id: u16,
        tick: u64,
    },
    TransformationEvent {
        stage: u8,
        tick: u64,
    },
    MemoryEvent {
        npc: CitizenId,
        about: CitizenId,
        impact: i8,
        tick: u64,
    },
    GossipEvent {
        speaker: CitizenId,
        listener: CitizenId,
        about: CitizenId,
        tick: u64,
    },
    EconomicTransaction {
        buyer: CitizenId,
        seller: CitizenId,
        resource: ResourceType,
        quantity: u32,
        price: f32,
        tick: u64,
    },
    CausalAction {
        causal: crate::types::CausalPointer,
        action_name: String,
        tick: u64,
    },
    ConsequenceMatured {
        consequence_id: u32,
        causal_root: u64,
        tick: u64,
    },
    KnowledgeShared {
        speaker: CitizenId,
        listener: CitizenId,
        knowledge_id: u16,
        corroboration: u8,
        tick: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeathCause {
    Starvation,
    OldAge,
    Injury,
}

// ── Telemetry Events (session-local, not persisted) ───────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub tick: u64,
    pub scenario_id: Option<u64>,
    pub event_type: TelemetryEventType,
    pub actor: Option<u64>,    // CitizenId.0
    pub target: Option<u64>,   // CitizenId.0
    pub location: Option<u16>, // LocationId.0
    pub action: Option<String>,
    pub outcome: Option<String>,
}

impl TelemetryEvent {
    pub fn player_action(tick: u64, action: &str, outcome: &str) -> Self {
        Self {
            tick,
            scenario_id: None,
            event_type: TelemetryEventType::PlayerAction,
            actor: Some(0),
            target: None,
            location: None,
            action: Some(action.to_string()),
            outcome: Some(outcome.to_string()),
        }
    }
}
