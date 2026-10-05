/// Godseed — TelemetryEventType (in types to break circular deps)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TelemetryEventType {
    PlayerAction,
    NpcGoalChanged,
    RelationshipChanged,
    EconomicTransaction,
    MarketPriceChanged,
    CapabilityAcquired,
    TransformationProgress,
    NpcMemoryFormed,
    GossipPropagated,
    PhysiologyEvent,
    DemographicEvent,
    SavePerformed,
    LoadPerformed,
    InvariantViolation,
    ErrorEvent,
}
