# GODSEED SYSTEMS ARCHITECTURE
## Phase 1 — Architecture Document

**Status**: `ARCHITECTURE_FROZEN`
**Revision**: 1.0.0
**Date**: 2026-09-29
**Basis**: CIVITAS-1M commit `0915514` + Godseed VS1 extensions

---

## 1. ARCHITECTURAL PHILOSOPHY

Godseed VS1 extends CIVITAS-1M's proven headless simulation with three new layers:

1. **Player Layer** — A unique player entity injected into the ECS world with
   player-specific components and an input queue.
2. **Social Intelligence Layer** — NPC memory, relationship ledgers, reputation
   system, and gossip propagation.
3. **Capability + Transformation Layer** — Capability acquisition, knowledge
   inventory, and the Inscription transformation path.

The terminal UI layer reads simulation state and writes to the player input queue.
It does NOT modify world state directly.

```
┌─────────────────────────────────────────────────────────┐
│                    TERMINAL UI LAYER                     │
│  (crossterm; reads snapshot; writes PlayerInputBuffer)   │
└──────────────────────┬──────────────────────────────────┘
                       │  PlayerInputBuffer (queue)
┌──────────────────────▼──────────────────────────────────┐
│              GODSEED SIMULATION ENGINE (Rust)            │
│                                                          │
│  ┌─────────────────────────────────────────────────┐    │
│  │              BEVY ECS WORLD                     │    │
│  │                                                  │    │
│  │  NPC Entities (25–40)   Player Entity (1)       │    │
│  │  ┌─────────────────┐   ┌──────────────────────┐ │    │
│  │  │ CitizenMeta     │   │ CitizenMeta          │ │    │
│  │  │ Demographics    │   │ Demographics         │ │    │
│  │  │ PhysicalNeeds   │   │ PhysicalNeeds        │ │    │
│  │  │ PersonalFinances│   │ PersonalFinances     │ │    │
│  │  │ OccupationProfile│  │ OccupationProfile   │ │    │
│  │  │ SettlementRef   │   │ SettlementRef        │ │    │
│  │  │ HouseholdRef    │   │ HouseholdRef         │ │    │
│  │  │ Kinship         │   │ Kinship              │ │    │
│  │  │ NpcMemory (NEW) │   │ PlayerMarker (NEW)   │ │    │
│  │  │ NpcSchedule(NEW)│   │ CapabilitySet (NEW)  │ │    │
│  │  │ NpcGoals (NEW)  │   │ TransformState (NEW) │ │    │
│  │  │ Disposition(NEW)│   │ KnowledgeInv (NEW)   │ │    │
│  │  └─────────────────┘   │ PlayerInputBuffer(NEW│ │    │
│  │                         └──────────────────────┘ │    │
│  │                                                  │    │
│  │  RESOURCES                                       │    │
│  │  ┌──────────────────────────────────────────┐   │    │
│  │  │ SimClock | WorldMap | SettlementDirectory│   │    │
│  │  │ HouseholdDirectory | EventRing           │   │    │
│  │  │ RelationshipLedger (NEW)                 │   │    │
│  │  │ ReputationRegistry (NEW)                 │   │    │
│  │  │ ContentDefinitions (NEW)                 │   │    │
│  │  │ TelemetryLog (NEW)                       │   │    │
│  │  └──────────────────────────────────────────┘   │    │
│  │                                                  │    │
│  │  SCHEDULES (multirate: Daily / Weekly / Monthly) │    │
│  │  Daily:  physiology, production, NPC routines,   │    │
│  │          NPC goals, player action processing,    │    │
│  │          memory consolidation, telemetry         │    │
│  │  Weekly: market, labor, gossip propagation,      │    │
│  │          relationship decay                      │    │
│  │  Monthly: migration, reproduction, reputation    │    │
│  │           normalization, transformation checks   │    │
│  └─────────────────────────────────────────────────┘    │
│                                                          │
│  PERSISTENCE (bincode + CRC32)                          │
│  REPLAY (event log + state hash)                        │
│  INVARIANT AUDIT (machine-checkable)                    │
└─────────────────────────────────────────────────────────┘
```

---

## 2. SYSTEM INVENTORY

### 2.1 Inherited from CIVITAS-1M (preserved or minimally adapted)

| System | Module | Responsibility | Schedule |
|--------|--------|---------------|----------|
| PhysiologySystem | `systems::physiology` | Satiety decay, health effects | Daily |
| ProductionSystem | `systems::production` | NPC labor → resource output | Daily |
| DemographicsSystem | `systems::demographics` | Aging, birth, death | Daily |
| MigrationTransitSystem | `systems::migration` | Move in-transit citizens | Daily |
| MarketPriceSystem | `systems::market` | Price discovery | Weekly |
| HouseholdConsumptionSystem | `systems::market` | Food/resource consumption | Weekly |
| LaborMarketSystem | `systems::labor` | Job assignment, wages | Weekly |
| MigrationEvaluationSystem | `systems::migration` | Migration push/pull | Monthly |
| ReproductionSystem | `systems::demographics` | Birth | Monthly |
| InvariantAudit | `invariants` | Machine-checkable constraints | On demand |
| Persistence | `persistence` | Save/load bincode snapshots | On command |
| Replay | `replay` | State hash, event log | On demand |

**Note**: MigrationEvaluationSystem is disabled for the single-settlement VS1
(no destination to migrate to within the settlement scope). Citizens may leave
but the primary focus is Thornveil. This is a scope boundary, not a bug.

### 2.2 New Godseed Systems

| System | Module | Responsibility | Schedule |
|--------|--------|---------------|----------|
| NpcRoutineSystem | `systems::npc_routine` | Execute NPC daily schedules (location, activity) | Daily |
| NpcGoalSystem | `systems::npc_goal` | Evaluate and update NPC short-term goals | Daily |
| PlayerActionSystem | `systems::player_action` | Drain PlayerInputBuffer, resolve player actions | Daily |
| NpcMemorySystem | `systems::npc_memory` | Consolidate events into NPC memories | Daily |
| TelemetrySystem | `systems::telemetry` | Emit structured telemetry events | Daily |
| GossipSystem | `systems::gossip` | Propagate observed events between NPCs | Weekly |
| RelationshipDecaySystem | `systems::relationship` | Apply passive relationship drift | Weekly |
| ReputationNormSystem | `systems::reputation` | Aggregate reputation per social group | Monthly |
| TransformationCheckSystem | `systems::transformation` | Check/advance transformation state | Monthly |

---

## 3. COMPONENT EXTENSIONS

### 3.1 Player-Only Components

```rust
/// Marker for the unique player entity
#[derive(Component)]
pub struct PlayerMarker;

/// Queue of pending player actions (written by UI, drained by PlayerActionSystem)
#[derive(Component)]
pub struct PlayerInputBuffer {
    pub queue: VecDeque<PlayerAction>,
}

/// Set of capabilities the player (or NPC) has acquired
#[derive(Component, Serialize, Deserialize)]
pub struct CapabilitySet {
    pub capabilities: HashMap<CapabilityId, CapabilityLevel>,
}

/// Current transformation path progress
#[derive(Component, Serialize, Deserialize)]
pub struct TransformationState {
    pub path: TransformationPath,
    pub stage: u8,           // 0 = not started, 1 = Scholar, 2 = Archivist, 3 = Living Record
    pub progress: u16,       // within current stage
    pub milestones: Vec<MilestoneId>, // achieved milestones
}

/// Facts and clues the player has learned
#[derive(Component, Serialize, Deserialize)]
pub struct KnowledgeInventory {
    pub nodes: HashSet<KnowledgeNodeId>,
}
```

### 3.2 NPC-Only Components

```rust
/// NPC memory of significant events
#[derive(Component, Serialize, Deserialize)]
pub struct NpcMemory {
    /// Recent events involving this NPC (capped at 20)
    pub events: VecDeque<MemoryEvent>,
    /// Known facts about other entities
    pub known_facts: HashMap<CitizenId, NpcFact>,
}

/// NPC daily routine definition
#[derive(Component, Serialize, Deserialize)]
pub struct NpcSchedule {
    /// List of (tick_in_day_start, activity) pairs
    pub slots: Vec<ScheduleSlot>,
    pub current_activity: NpcActivity,
    pub current_location: LocationId,
}

/// NPC current goals (short-term)
#[derive(Component, Serialize, Deserialize)]
pub struct NpcGoals {
    pub active_goal: Option<NpcGoal>,
    pub pending_goals: Vec<NpcGoal>,
    pub satisfied_needs: u8,
}

/// NPC disposition toward player
#[derive(Component, Serialize, Deserialize)]
pub struct Disposition {
    pub toward_player: i16,       // -100 to +100
    pub base_disposition: i16,    // influenced by reputation
    pub suspicion: u8,            // 0–100; theft/damage evidence
}
```

### 3.3 New Resources

```rust
/// Tracks all player–NPC and NPC–NPC relationship values
pub struct RelationshipLedger {
    pub values: HashMap<(CitizenId, CitizenId), i16>, // -100..+100
}

/// Aggregate reputation per social group
pub struct ReputationRegistry {
    pub groups: HashMap<SocialGroupId, ReputationRecord>,
}

/// Content definitions (NPC personas, locations, capabilities, knowledge nodes)
pub struct ContentDefinitions {
    pub npc_definitions: Vec<NpcDefinition>,
    pub location_definitions: Vec<LocationDefinition>,
    pub capability_definitions: Vec<CapabilityDefinition>,
    pub knowledge_nodes: Vec<KnowledgeNode>,
    pub transformation_paths: Vec<TransformationPathDefinition>,
}

/// Structured telemetry event log
pub struct TelemetryLog {
    pub events: VecDeque<TelemetryEvent>, // capped at 50_000
}
```

---

## 4. STATE OWNERSHIP

| State | Owner | Persistence |
|-------|-------|-------------|
| Player entity components | Bevy ECS World | Serialized in snapshot |
| NPC entity components | Bevy ECS World | Serialized in snapshot |
| Settlement data | SettlementDirectory (Resource) | Serialized in snapshot |
| Household data | HouseholdDirectory (Resource) | Serialized in snapshot |
| Relationship values | RelationshipLedger (Resource) | Serialized in snapshot |
| Reputation | ReputationRegistry (Resource) | Serialized in snapshot |
| Market prices | SettlementDirectory.markets | Serialized in snapshot |
| Telemetry | TelemetryLog (Resource) | NOT persisted (session-local) |
| Content definitions | ContentDefinitions (Resource) | Loaded from content files at start |
| Player input queue | PlayerInputBuffer (Component) | NOT persisted (ephemeral) |
| World map | WorldMap (Resource) | Serialized in snapshot |
| Sim clock | SimClock (Resource) | Serialized in snapshot |

---

## 5. UPDATE / TICK FLOW

```
Terminal Input Thread
    │  writes PlayerAction to PlayerInputBuffer
    ▼
Simulation Thread (main)
    │
    ├── Every Tick (Daily):
    │   1. NpcRoutineSystem    — move NPCs per schedule
    │   2. PlayerActionSystem  — drain input queue, resolve actions
    │   3. ProductionSystem    — NPC labor produces resources
    │   4. PhysiologySystem    — satiety/health decay (NPCs + player)
    │   5. NpcGoalSystem       — NPCs evaluate goals
    │   6. NpcMemorySystem     — consolidate events to memory
    │   7. DemographicsSystem  — aging
    │   8. TelemetrySystem     — emit structured events
    │
    ├── Every 7 Ticks (Weekly):
    │   1. MarketPriceSystem   — price discovery
    │   2. HouseholdConsumptionSystem — resource consumption
    │   3. LaborMarketSystem   — job / wage updates
    │   4. GossipSystem        — propagate observations between NPCs
    │   5. RelationshipDecaySystem — passive drift
    │
    └── Every 30 Ticks (Monthly):
        1. MigrationEvaluationSystem (reduced scope for VS1)
        2. ReproductionSystem
        3. ReputationNormSystem
        4. TransformationCheckSystem — check if transformation conditions met
```

---

## 6. PLAYER ACTION SYSTEM

The PlayerActionSystem is the bridge between UI input and world state.

**Action Resolution Pipeline**:
1. Dequeue one `PlayerAction` from `PlayerInputBuffer`.
2. Validate action legality (location, resources, capability, relationship threshold).
3. Execute action consequences (modify components, emit events, update memory).
4. Return `ActionResult` to UI display buffer.
5. Emit telemetry event.

**Action Categories**:

| Category | Examples |
|----------|---------|
| Movement | `Move(LocationId)` |
| Observation | `Inspect(EntityId)`, `Look` |
| Social | `Talk(NpcId, TopicId)`, `Offer(NpcId, ExchangeId)` |
| Economic | `Buy(ItemId, quantity)`, `Sell(ItemId, quantity)`, `Work(OccupationId)` |
| Object | `PickUp(ObjectId)`, `Drop(ObjectId)`, `Use(ObjectId)` |
| Capability | `Practice(CapabilityId)`, `LearnFrom(NpcId, CapabilityId)` |
| Transformation | `Inscribe(ObservationId)`, `StudyArchive`, `Experiment` |
| Time | `Wait(ticks)`, `Sleep` |

---

## 7. PERSISTENCE BOUNDARIES

### Serialized in Snapshot (must survive save/load)
- All ECS entity components (player + NPCs)
- All Resource state (SimClock, WorldMap, SettlementDirectory, HouseholdDirectory,
  RelationshipLedger, ReputationRegistry, EventRing, NextCitizenId)
- ContentDefinitions are re-loaded from content files on startup (content is
  stable between saves unless the campaign advances)

### NOT Serialized (ephemeral)
- PlayerInputBuffer (flushed before save)
- TelemetryLog (session-local)
- Terminal UI display buffers

### Integrity
- Every snapshot: `GODSEED1` magic header + u32 CRC32 over all data.
- Corrupted or truncated snapshots are rejected at load with explicit error.

---

## 8. WORLD MAP AND SETTLEMENT LAYOUT

### Thornveil Map (authored, deterministic)
- Grid-based map: 20×20 cells
- Cell types: `Road`, `Building`, `Field`, `Forest`, `Water`, `Open`
- Named locations: Inn (The Slanted Timber), Forge, Market Square,
  Farmfields (N, S), Herb Garden, The Archive (initially empty/ruined),
  Forest Edge, Well, Storage House
- NPCs have home locations and work locations; schedule slots move them
  between these during the day

### NPC Identity (authored, content-defined)
25–35 NPCs with authored names, occupations, households, initial dispositions.
NPC behaviors are systemic; names and occupations are authored content.

Example NPCs (authoritative content in `content/thornveil.toml`):
- Mira Ashbridge — innkeeper, female, household head, relationship hub
- Wren Forscythe — blacksmith, male, skill-teaches Smithing at relationship > 40
- Oswin Cley — farmer, male, primary food producer
- Sera Cley — farmer/herbalist, female, spouse of Oswin, teaches Herbalism at > 50
- Elder Voss — settlement elder, male, very old, holds Inscription knowledge clue

---

## 9. TRANSFORMATION SYSTEM — INSCRIPTION PATH

### Architecture

```
TransformationPath::Inscription
    Stage 0 (Uninitiated)
        → prerequisite: discover MemoryNode::AncientArchive AND talk to Elder Voss
    Stage 1 (Scholar)
        → actions enabled: Inscribe, StudyArchive
        → NPC reaction: "the one who records"
        → milestones: 5 inscriptions completed
    Stage 2 (Archivist) [SEAMED — not fully implemented in VS1]
        → NPC memory queries player archive
    Stage 3 (Living Record) [SEAMED — conceptual only in VS1]
```

### Transformation Check (Monthly)
```rust
fn transformation_check_system(
    player: Query<(&PlayerMarker, &TransformationState, &KnowledgeInventory, &CapabilitySet)>,
    relationships: Res<RelationshipLedger>,
    npcs: Query<(&CitizenMeta, &NpcMemory)>,
    mut events: EventWriter<TransformationProgressEvent>,
)
```

### Milestone System
Each milestone has:
- Required conditions (knowledge nodes, capabilities, relationship thresholds, actions completed)
- Effects (new capability unlocked, NPC reaction rule added, UI message)
- Persistence (stored in TransformationState.milestones)

---

## 10. TELEMETRY SCHEMA

```rust
pub struct TelemetryEvent {
    pub tick: u64,
    pub scenario_id: Option<u64>,      // for life tests
    pub event_type: TelemetryEventType,
    pub actor: Option<CitizenId>,
    pub target: Option<CitizenId>,
    pub location: Option<LocationId>,
    pub pre_state: Option<String>,     // compact JSON
    pub action: Option<String>,
    pub post_state: Option<String>,    // compact JSON
    pub causal_link: Option<String>,
}

pub enum TelemetryEventType {
    PlayerAction, NpcGoalChanged, RelationshipChanged,
    EconomicTransaction, MarketPriceChanged, CapabilityAcquired,
    TransformationProgress, NpcMemoryFormed, GossipPropagated,
    PhysiologyEvent, DemographicEvent, SavePerformed, LoadPerformed,
    InvariantViolation, ErrorEvent,
}
```

---

## 11. REQUIREMENT → SYSTEM TRACEABILITY

| Acceptance Criterion | Responsible System(s) | Verification Mechanism |
|----------------------|-----------------------|----------------------|
| AC-1: Settlement | ContentDefinitions + WorldMap | Invariant: settlement exists with 25–40 NPCs |
| AC-2: Player Entity | PlayerMarker components | Invariant: exactly one PlayerMarker entity |
| AC-3: Movement/Interaction | PlayerActionSystem + WorldMap | Scripted movement test |
| AC-4: NPC Autonomy | NpcRoutineSystem + NpcGoalSystem + PhysiologySystem | Observe NPC location changes per tick |
| AC-5: NPC Memory | NpcMemorySystem + GossipSystem | Memory persistence test |
| AC-6: Relationships | RelationshipLedger + RelationshipDecaySystem | Relationship threshold unlock test |
| AC-7: Economy | Market/Labor/Production systems | Economy exploit test + price response test |
| AC-8: Capability | PlayerActionSystem + CapabilitySet | Capability acquisition test |
| AC-9: Transformation | TransformationCheckSystem + TransformationState | Transformation milestone test |
| AC-10: Persistence | Persistence module | Save/Load/Hash roundtrip test |
| AC-11: Time | SimClock + all schedule systems | 30-day skip test |
| AC-12: Consequences | All simulation systems | Return-after-absence test |
| AC-13: Playthroughs | Life test harness | 5-persona divergence test |
| AC-14: Fun Hypotheses | Life test harness + telemetry | FH evidence collection |

---

## 12. KNOWN TECHNICAL RISKS

| Risk | Severity | Mitigation |
|------|----------|-----------|
| NPC memory growth unbounded | Medium | Cap at 20 events per NPC; rolling dequeue |
| Relationship ledger O(n²) | Low | 30 NPCs → 900 pairs → trivial; not a concern at this scale |
| Terminal UI blocking simulation | Medium | Input processed async; UI reads from display buffer, not world |
| Transformation conditions too opaque | Medium | Telemetry + knowledge system reveals clues |
| Economy collapse with 30 NPCs | Medium | Tune constants from CIVITAS-1M benchmarks |
| ECS borrow conflicts (player + NPC same query) | Low | Player system uses separate query filtered by PlayerMarker |
| Save file compatibility across contract changes | Medium | Bump save version on any schema change; reject old saves cleanly |
| NPC gossip creating infinite loop | Low | Gossip is one-hop per weekly tick; capped propagation |

---

## PHASE 1 GATE DISPOSITION

`GODSEED_PHASE_1_ARCHITECTURE_PASS`

Every AC maps to a responsible system and verification mechanism.

No architectural dead end was identified.

No scale risk exists at 30-NPC scope.

Implementation may begin.

---

*Architecture frozen: 2026-09-29T07:35:00-07:00*
