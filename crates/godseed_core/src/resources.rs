/// Godseed — ECS Resources (global simulation state)

use bevy_ecs::system::Resource;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

use crate::types::{CitizenId, ReputationRecord};
use crate::events::{SimEvent, TelemetryEvent};

// ── Relationship Ledger ───────────────────────────────────────────────────────

/// All pairwise relationship values (player ↔ NPC and NPC ↔ NPC)
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationshipLedger {
    pub values: HashMap<(u64, u64), i16>, // (CitizenId.0, CitizenId.0) → -100..+100
}

impl RelationshipLedger {
    pub fn get(&self, a: CitizenId, b: CitizenId) -> i16 {
        let key = Self::key(a, b);
        *self.values.get(&key).unwrap_or(&0)
    }

    pub fn adjust(&mut self, a: CitizenId, b: CitizenId, delta: i16) {
        let key = Self::key(a, b);
        let v = self.values.entry(key).or_insert(0);
        *v = (*v + delta).clamp(-100, 100);
    }

    pub fn set(&mut self, a: CitizenId, b: CitizenId, value: i16) {
        let key = Self::key(a, b);
        self.values.insert(key, value.clamp(-100, 100));
    }

    fn key(a: CitizenId, b: CitizenId) -> (u64, u64) {
        if a.0 <= b.0 { (a.0, b.0) } else { (b.0, a.0) }
    }
}

// ── Reputation Registry ───────────────────────────────────────────────────────

/// Aggregate settlement reputation per social group
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReputationRegistry {
    pub player_reputation: i16, // overall player reputation in Thornveil (-100..+100)
    pub groups: HashMap<u8, ReputationRecord>, // SocialGroupId.0 → record
}

impl ReputationRegistry {
    pub fn adjust_player(&mut self, delta: i16) {
        self.player_reputation = (self.player_reputation + delta).clamp(-100, 100);
    }
}

// ── Event Ring (inherited from CIVITAS-1M) ────────────────────────────────────


#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct EventRing {
    pub events: VecDeque<SimEvent>,
    pub capacity: usize,
    pub total_emitted: u64,
}

impl EventRing {
    pub fn new(capacity: usize) -> Self {
        Self { events: VecDeque::new(), capacity, total_emitted: 0 }
    }

    pub fn emit(&mut self, event: SimEvent) {
        if self.events.len() >= self.capacity {
            self.events.pop_front();
        }
        self.events.push_back(event);
        self.total_emitted += 1;
    }

    pub fn recent(&self, n: usize) -> impl Iterator<Item = &SimEvent> {
        let start = self.events.len().saturating_sub(n);
        self.events.range(start..)
    }
}

// ── Telemetry Log ─────────────────────────────────────────────────────────────

#[derive(Resource, Debug, Clone, Default)]
pub struct TelemetryLog {
    pub events: VecDeque<TelemetryEvent>,
    pub scenario_id: Option<u64>,
}

impl TelemetryLog {
    pub fn new() -> Self { Self::default() }

    pub fn emit(&mut self, event: TelemetryEvent) {
        if self.events.len() >= 50_000 {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    /// Dump all events to JSON lines string
    pub fn to_jsonl(&self) -> String {
        use serde_json::to_string;
        self.events.iter()
            .filter_map(|e| to_string(e).ok())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// ── Next Citizen ID counter ───────────────────────────────────────────────────

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NextCitizenId(pub u64);
