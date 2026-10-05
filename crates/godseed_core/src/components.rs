/// Godseed Core — ECS Components
///
/// All ECS components for both NPC and Player entities.
/// Player-specific components are marked; most are shared with NPCs.
use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::types::{
    CapabilityId, CapabilityLevel, CitizenId, DecisionTrace, EpisodicRecord, Gender, HouseholdId,
    HouseholdRole, KnowledgeNodeId, LocationId, MemoryEvent, MemoryTag, MigrationStatus,
    MilestoneId, NpcActivity, NpcFact, NpcGoal, OccupationType, PlayerAction, RelationalBond,
    ScheduleSlot, SettlementId, TransformationPath,
};

// ── Shared Components (NPC + Player) ─────────────────────────────────────────

/// Identity and vital status
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitizenMeta {
    pub id: CitizenId,
    pub name: String,
    pub gender: Gender,
    pub alive: bool,
}

/// Age and health
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Demographics {
    pub age_years: u16,
    pub age_ticks: u32,
    pub health: u8, // 0–100
    pub fertility_timer: u8,
}

/// Household membership
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdRef {
    pub household_id: HouseholdId,
    pub role: HouseholdRole,
}

/// Settlement and location
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementRef {
    pub settlement_id: SettlementId,
    pub current_location: LocationId,
}

/// Occupation and work capability
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OccupationProfile {
    pub occupation: OccupationType,
    pub skill_level: u8,
    pub experience: u32,
    pub productivity: f32,
}

/// Personal finances
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalFinances {
    pub coins: f64,
    pub last_income: f32,
    pub last_expense: f32,
    pub debt: f64,
}

/// Physical needs — shared by NPCs and player
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhysicalNeeds {
    pub satiety: u8, // 0–100; 0 = starving, 100 = full
    pub shelter: u8, // 0–100
    pub rest: u8,    // 0–100; 0 = exhausted
}

/// Migration/mobility status
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilityProfile {
    pub status: MigrationStatus,
}

/// Family relations
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Kinship {
    pub spouse: Option<CitizenId>,
    pub parent_a: Option<CitizenId>,
    pub parent_b: Option<CitizenId>,
    pub children_count: u16,
}

/// Last decision trace for causality inspection
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CausalAudit {
    pub trace: DecisionTrace,
}

/// Personal inventory (portable resources)
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    /// resource_type → quantity
    pub items: HashMap<u8, u32>, // using u8 key for ResourceType ordinal to support serde easily
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn get(&self, resource_ordinal: u8) -> u32 {
        *self.items.get(&resource_ordinal).unwrap_or(&0)
    }

    pub fn add(&mut self, resource_ordinal: u8, quantity: u32) {
        *self.items.entry(resource_ordinal).or_insert(0) += quantity;
    }

    pub fn remove(&mut self, resource_ordinal: u8, quantity: u32) -> bool {
        let current = self.get(resource_ordinal);
        if current < quantity {
            return false;
        }
        *self.items.entry(resource_ordinal).or_insert(0) -= quantity;
        true
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

// ── NPC-Only Components ───────────────────────────────────────────────────────

/// NPC's memory of significant events (especially concerning the player)
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcMemory {
    /// Rolling window of last 20 significant events
    pub events: VecDeque<MemoryEvent>,
    /// Facts the NPC knows about other entities
    pub known_facts: HashMap<u64, NpcFact>, // CitizenId.0 → fact
}

impl NpcMemory {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            known_facts: HashMap::new(),
        }
    }

    pub fn add_event(&mut self, event: MemoryEvent) {
        if self.events.len() >= 20 {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    /// Sum of impact from all events involving a given subject
    pub fn disposition_toward(&self, subject: CitizenId) -> i16 {
        self.events
            .iter()
            .filter(|e| e.subject == subject)
            .map(|e| e.impact as i16)
            .sum()
    }
}

impl Default for NpcMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// NPC daily schedule (what they do and where)
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcSchedule {
    pub slots: Vec<ScheduleSlot>,
    pub current_activity: NpcActivity,
    pub home_location: LocationId,
    pub work_location: LocationId,
}

impl NpcSchedule {
    /// Get the appropriate activity for a given hour (0–23)
    pub fn activity_at_hour(&self, hour: u64) -> (NpcActivity, LocationId) {
        // Scan slots in reverse to find the last slot that started at or before this hour
        let mut activity = NpcActivity::Idle;
        let mut location = self.home_location;
        for slot in &self.slots {
            if (slot.tick_start as u64) <= hour {
                activity = slot.activity;
                location = slot.location;
            }
        }
        (activity, location)
    }
}

/// NPC short-term goals
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcGoals {
    pub active_goal: Option<NpcGoal>,
    pub goal_ticks_remaining: u32,
}

impl NpcGoals {
    pub fn new() -> Self {
        Self {
            active_goal: None,
            goal_ticks_remaining: 0,
        }
    }
}

impl Default for NpcGoals {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy V1/V2 persistence migration structure. Discarded from V2/V3 runtime state.
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Disposition {
    /// Memory-derived disposition toward the player (-100 to +100)
    pub toward_player: i16,
    /// Baseline from initial personality (-20 to +20)
    pub base_personality: i8,
    /// Suspicion level from witnessed transgressions (0–100)
    pub suspicion: u8,
    /// Whether this NPC will teach capabilities (requires relationship)
    pub will_teach: Option<CapabilityId>,
    /// Minimum relationship required to unlock teaching
    pub teach_threshold: i16,
}

impl Disposition {
    pub fn new(base: i8) -> Self {
        Self {
            toward_player: base as i16,
            base_personality: base,
            suspicion: 0,
            will_teach: None,
            teach_threshold: 40,
        }
    }

    /// Effective relationship (memory-derived + base)
    pub fn effective_relationship(&self) -> i16 {
        (self.toward_player + self.base_personality as i16).clamp(-100, 100)
    }
}

/// Static authored social and teaching profile (immutable design parameters, Sec 11)
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcSocialProfile {
    /// Baseline from initial personality (-20 to +20)
    pub base_personality: i8,
    /// Suspicion baseline from background (0–100)
    pub base_suspicion: u8,
    /// Whether this NPC will teach capabilities
    pub will_teach: Option<CapabilityId>,
    /// Minimum relationship required to unlock teaching
    pub teach_threshold: i16,
}

impl NpcSocialProfile {
    pub fn new(base: i8) -> Self {
        Self {
            base_personality: base,
            base_suspicion: 0,
            will_teach: None,
            teach_threshold: 40,
        }
    }
}

impl Default for NpcSocialProfile {
    fn default() -> Self {
        Self::new(0)
    }
}

// ── VS2 Relational Ledger & Episodic Memory Components ───────────────────────

/// Entity-local Relational Ledger holding triad bonds (AC-201)
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RelationalLedger {
    pub bonds: HashMap<u64, RelationalBond>,
}

impl RelationalLedger {
    pub fn new() -> Self {
        Self {
            bonds: HashMap::new(),
        }
    }

    pub fn get_bond(&self, target: CitizenId) -> RelationalBond {
        *self
            .bonds
            .get(&target.0)
            .unwrap_or(&RelationalBond::default())
    }

    pub fn get_bond_mut(&mut self, target: CitizenId) -> &mut RelationalBond {
        self.bonds
            .entry(target.0)
            .or_insert(RelationalBond::default())
    }

    pub fn set_bond(&mut self, target: CitizenId, bond: RelationalBond) {
        self.bonds.insert(target.0, bond);
    }

    pub fn adjust(&mut self, target: CitizenId, delta_s: i8, delta_t: i8, delta_o: i16) {
        let bond = self
            .bonds
            .entry(target.0)
            .or_insert(RelationalBond::default());
        bond.sentiment = (bond.sentiment as i16 + delta_s as i16).clamp(-100, 100) as i8;
        bond.trust = (bond.trust as i16 + delta_t as i16).clamp(-100, 100) as i8;
        bond.obligation = (bond.obligation + delta_o).clamp(-1000, 1000);
    }
}

/// Compact permanent anchor preserving semantic fact indefinitely under anchor pressure (AC-202, Option A)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactAnchor {
    pub id: u64,
    pub tick: u64,
    pub actor: CitizenId,
    pub target: Option<CitizenId>,
    pub tag: MemoryTag,
    pub narrative_token: u16,
    pub causal: Option<crate::types::CausalPointer>,
}

/// Dual-stream bounded episodic memory (12 transient FIFO + 6 active anchors + compact permanent storage) (AC-202)
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub transient: VecDeque<EpisodicRecord>,
    pub anchors: Vec<EpisodicRecord>,
    #[serde(default)]
    pub compacted_anchors: Vec<CompactAnchor>,
}

impl EpisodicMemory {
    pub fn new() -> Self {
        Self {
            transient: VecDeque::new(),
            anchors: Vec::new(),
            compacted_anchors: Vec::new(),
        }
    }

    pub fn add_record(&mut self, record: EpisodicRecord) {
        if record.is_permanent {
            if self.anchors.len() >= 6 {
                // Evict anchor with smallest absolute delta impact into compact permanent storage (Option A)
                if let Some((min_idx, _)) = self.anchors.iter().enumerate().min_by_key(|(_, r)| {
                    r.delta_sentiment.abs() as i32 + r.delta_trust.abs() as i32
                }) {
                    let evicted = self.anchors.remove(min_idx);
                    self.compacted_anchors.push(CompactAnchor {
                        id: evicted.id,
                        tick: evicted.tick,
                        actor: evicted.actor,
                        target: evicted.target,
                        tag: evicted.tag,
                        narrative_token: evicted.narrative_token,
                        causal: evicted.causal,
                    });
                }
            }
            self.anchors.push(record);
        } else {
            self.add_transient(record);
        }
    }

    fn add_transient(&mut self, record: EpisodicRecord) {
        if self.transient.len() >= 12 {
            self.transient.pop_front();
        }
        self.transient.push_back(record);
    }

    pub fn has_anchor_with_tag(&self, tag: MemoryTag) -> bool {
        self.anchors.iter().any(|a| a.tag == tag)
            || self.compacted_anchors.iter().any(|c| c.tag == tag)
    }

    pub fn has_record_with_tag(&self, tag: MemoryTag) -> bool {
        self.has_anchor_with_tag(tag) || self.transient.iter().any(|t| t.tag == tag)
    }

    pub fn find_anchor(&self, tag: MemoryTag) -> Option<&EpisodicRecord> {
        self.anchors.iter().find(|a| a.tag == tag)
    }

    pub fn find_record_with_tag(&self, tag: MemoryTag) -> Option<&EpisodicRecord> {
        self.anchors
            .iter()
            .find(|a| a.tag == tag)
            .or_else(|| self.transient.iter().find(|t| t.tag == tag))
    }

    pub fn find_compact_anchor(&self, tag: MemoryTag) -> Option<&CompactAnchor> {
        self.compacted_anchors.iter().find(|c| c.tag == tag)
    }

    pub fn all_records(&self) -> impl Iterator<Item = &EpisodicRecord> {
        self.anchors.iter().chain(self.transient.iter())
    }
}

impl Default for EpisodicMemory {
    fn default() -> Self {
        Self::new()
    }
}

// ── Player-Only Components ─────────────────────────────────────────────────────

/// Marker: the unique player entity
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlayerMarker;

/// Queue of pending player actions (UI writes here; PlayerActionSystem drains it)
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInputBuffer {
    pub queue: VecDeque<PlayerAction>,
    /// Results from last processed action (read by UI)
    pub last_results: VecDeque<crate::types::ActionResult>,
}

impl PlayerInputBuffer {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            last_results: VecDeque::new(),
        }
    }

    pub fn push_action(&mut self, action: PlayerAction) {
        self.queue.push_back(action);
    }

    pub fn pop_action(&mut self) -> Option<PlayerAction> {
        self.queue.pop_front()
    }

    pub fn push_result(&mut self, result: crate::types::ActionResult) {
        if self.last_results.len() >= 50 {
            self.last_results.pop_front();
        }
        self.last_results.push_back(result);
    }
}

impl Default for PlayerInputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Set of capabilities the player (or NPC) has acquired
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilitySet {
    pub capabilities: HashMap<u16, CapabilityLevel>, // CapabilityId.0 → level
    /// Practice ticks accumulated toward next level for current practice
    pub practice_progress: HashMap<u16, u32>,
}

impl CapabilitySet {
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
            practice_progress: HashMap::new(),
        }
    }

    pub fn has(&self, id: CapabilityId) -> bool {
        self.capabilities
            .get(&id.0)
            .map(|l| l.0 > 0)
            .unwrap_or(false)
    }

    pub fn level(&self, id: CapabilityId) -> CapabilityLevel {
        *self
            .capabilities
            .get(&id.0)
            .unwrap_or(&CapabilityLevel::NONE)
    }

    pub fn set(&mut self, id: CapabilityId, level: CapabilityLevel) {
        self.capabilities.insert(id.0, level);
    }

    pub fn add_practice(&mut self, id: CapabilityId, ticks: u32) -> bool {
        let progress = self.practice_progress.entry(id.0).or_insert(0);
        *progress += ticks;
        // Level up every 100 practice ticks (up to SKILLED)
        let current = *self
            .capabilities
            .get(&id.0)
            .unwrap_or(&CapabilityLevel::NONE);
        if *progress >= 100 && current < CapabilityLevel::SKILLED {
            *progress = 0;
            self.capabilities
                .insert(id.0, CapabilityLevel(current.0 + 1));
            return true; // leveled up
        }
        false
    }
}

impl Default for CapabilitySet {
    fn default() -> Self {
        Self::new()
    }
}

/// Current transformation path and stage
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransformationState {
    pub path: TransformationPath,
    pub stage: u8,     // 0 = uninitiated, 1 = Scholar, 2+ = seamed
    pub progress: u16, // within current stage (0–100)
    pub milestones: Vec<MilestoneId>,
    pub inscriptions_completed: u32,
    pub archive_studied_count: u32,
}

impl TransformationState {
    pub fn new() -> Self {
        Self {
            path: TransformationPath::None,
            stage: 0,
            progress: 0,
            milestones: Vec::new(),
            inscriptions_completed: 0,
            archive_studied_count: 0,
        }
    }

    pub fn is_scholar(&self) -> bool {
        self.path == TransformationPath::Inscription && self.stage >= 1
    }
}

impl Default for TransformationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Knowledge the player has learned about the world
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeInventory {
    pub nodes: HashSet<u32>, // KnowledgeNodeId.0
}

impl KnowledgeInventory {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
        }
    }

    pub fn learn(&mut self, node: KnowledgeNodeId) -> bool {
        self.nodes.insert(node.0)
    }

    pub fn knows(&self, node: KnowledgeNodeId) -> bool {
        self.nodes.contains(&node.0)
    }
}

impl Default for KnowledgeInventory {
    fn default() -> Self {
        Self::new()
    }
}

/// Epistemic state tracking factual knowledge nodes and corroboration (AC-204, Sec 8)
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EpistemicState {
    /// KnowledgeId (u16) -> (AcquiredTick, CorroborationCount)
    pub known: HashMap<u16, (u64, u8)>,
}

impl EpistemicState {
    pub fn new() -> Self {
        Self {
            known: HashMap::new(),
        }
    }

    pub fn has_knowledge(&self, id: u16) -> bool {
        self.known.contains_key(&id)
    }

    pub fn get_corroboration(&self, id: u16) -> u8 {
        self.known.get(&id).map(|(_, c)| *c).unwrap_or(0)
    }

    /// Learn or corroborate knowledge. Returns true if novel or corroboration increased.
    /// Returns false if already max corroborated (no-op suppression).
    pub fn learn(&mut self, id: u16, tick: u64) -> bool {
        if let Some((_, count)) = self.known.get_mut(&id) {
            if *count < 3 {
                *count += 1;
                true
            } else {
                false // No-op suppression (already saturated at max corroboration)
            }
        } else {
            self.known.insert(id, (tick, 1));
            true
        }
    }
}
