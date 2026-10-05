# GODSEED VERTICAL SLICE 2: SYSTEMS ARCHITECTURE
## Technical Architecture for Meaning, Attachment & Consequence

**Document Identifier:** `GODSEED-VS2-ARCH-001`  
**Governing Product Basis:** [GODSEED_VS2_PRODUCT_CONTRACT_REVISED.md](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/GODSEED_VS2_PRODUCT_CONTRACT_REVISED.md) (`E9CD4A2A79AFCA4970D301A41619B9639A761A82630194296CC18EEE2CE72296`)  
**Product Review Reference:** [GODSEED_VS2_PRODUCT_REVIEW.md](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/GODSEED_VS2_PRODUCT_REVIEW.md) (`26569193B75E15A912E7A4EC79901FAEC07295FBD879AF6CF16677C11A61348B`)  
**Baseline Candidate:** `391d2937379d775d12da2425001ea2fc88b733b4` (VS1 Candidate)  
**Lifecycle State:** `ARCHITECTURE_REVIEW`  
**Authority Disposition:** `ARCHITECTURE_READY_FOR_REVIEW` (Implementation Authorized: `NO`)  

---

## 1. Architecture Goals

The primary goal of the Godseed VS2 architecture is to implement the approved Product Contract with the absolute minimum technical machinery required to deliver the target player experience.

Architecture exists solely to serve the six-step universal experience loop:
$$\text{Observe \& Connect} \rightarrow \text{Encounter Tension} \rightarrow \text{Intervene/Refrain} \rightarrow \text{Live with Ripple} \rightarrow \text{Experience Absent World} \rightarrow \text{Reap Delayed Consequence}$$

### Core Architectural Invariants:
1. **Determinism:** Bit-identical simulation state from any identical seed and input stream.
2. **State Boundedness:** $O(1)$ memory growth per NPC over infinite simulation ticks (no memory leaks during 90-day soak testing).
3. **Inspectable Lineage:** Every delayed consequence and NPC disposition shift must be traceable to a specific prior cause.
4. **Generalizability:** Primitives designed for VS2 (relationships, memory, knowledge, off-screen continuation) must serve future progression paths (Warrior, Ruler, Explorer, Mystic) without requiring redesign.

---

## 2. Product Constraints & Hard Boundaries

From `GODSEED_VS2_PRODUCT_CONTRACT_REVISED.md`, the architecture operates under strict non-negotiable boundaries:
- **Spatial Scope:** Exactly 11 spatial nodes within Thornveil Settlement. No outer regions or multi-settlement maps.
- **Population Scope:** Exactly the 15 authored citizens from VS1. Zero procedural NPCs.
- **Inhabitant Tiers:** 4 Anchor NPCs, 5 Focal NPCs, 6 Texture NPCs. The architecture must provide uniform ECS component interfaces while supporting tiered content depth.
- **No Generative AI Runtime:** Zero non-deterministic LLM calls in the simulation or dialogue pipelines.
- **No Combat Subsystems:** Zero hit-point damage, weapons, or military mechanics.
- **Save Integrity:** Strict persistence round-tripping with schema evolution from `GODSEED1` to `GODSEED2`.

---

## 3. VS1 Baseline & Evolution Strategy

VS1 established a high-throughput, headless Bevy ECS engine (~600,000 ticks/sec) operating on hourly ticks (24 ticks = 1 day), with daily, weekly, and monthly schedules.

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        VS1 → VS2 SYSTEM DELTAS                         │
├──────────────────────────┬──────────────┬──────────────────────────────┤
│ System                   │ Status       │ Architectural Delta          │
├──────────────────────────┼──────────────┼──────────────────────────────┤
│ PhysiologySystem         │ UNCHANGED    │ Satiety/health decay         │
│ ProductionSystem         │ UNCHANGED    │ Labor → commodity output     │
│ MarketPriceSystem        │ UNCHANGED    │ Elastic supply/demand pricing│
│ NpcRoutineSystem         │ EXTENDED     │ Supports dynamic work slots  │
│ RelationshipSystem       │ REPLACED     │ Triad bond (Sentiment/Trust/ │
│                          │              │ Obligation) + Behavioral Mode│
│ NpcMemorySystem          │ REPLACED     │ Bounded episodic records +   │
│                          │              │ permanent anchor retention   │
│ GossipSystem             │ REPLACED     │ One-hop narrative gossip     │
│ KnowledgeSystem          │ EXTENDED     │ Asymmetric EpistemicState    │
│ PlayerActionSystem       │ EXTENDED     │ Adds Diagnose & InscribeDoc  │
│ SocialVectorSystem       │ NEW          │ Off-screen continuation &    │
│                          │              │ delayed consequence pipeline │
│ ReturnDigestSystem       │ NEW          │ Generates narrative return   │
│                          │              │ salutations upon arrival     │
└──────────────────────────┴──────────────┴──────────────────────────────┘
```

---

## 4. System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           TERMINAL UI LAYER                             │
│       (crossterm; renders viewports, dialogue, and return digests)       │
└────────────────────────────────────┬────────────────────────────────────┘
                                     │  PlayerAction queue
┌────────────────────────────────────▼────────────────────────────────────┐
│                    GODSEED SIMULATION ENGINE (Bevy ECS)                 │
│                                                                         │
│  ENTITIES (1 Player + 15 Inhabitants)                                  │
│  ┌─────────────────────────┐         ┌───────────────────────────────┐ │
│  │     COMMON CITIZEN      │         │         PLAYER ENTITY         │ │
│  ├─────────────────────────┤         ├───────────────────────────────┤ │
│  │ CitizenMeta             │         │ CitizenMeta                   │ │
│  │ Demographics            │         │ PhysicalNeeds                 │ │
│  │ PhysicalNeeds           │         │ Inventory                     │ │
│  │ PersonalFinances        │         │ CapabilitySet                 │ │
│  │ Inventory               │         │ EpistemicState (NEW)          │ │
│  │ SettlementRef           │         │ TransformationState           │ │
│  │ HouseholdRef            │         │ PlayerInputBuffer             │ │
│  │ NpcSchedule             │         │ PlayerMarker                  │ │
│  │ EpisodicMemory (NEW)    │         └───────────────────────────────┘ │
│  │ EpistemicState (NEW)    │                                           │
│  │ RelationalLedger (NEW)  │                                           │
│  │ NpcGoals                │                                           │
│  └─────────────────────────┘                                           │
│                                                                         │
│  RESOURCES                                                              │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │ SimClock              | WorldMap          | SettlementDirectory  │  │
│  │ HouseholdDirectory    | EventRing         | ContentDefinitions   │  │
│  │ SocialVectorRegistry (NEW)                | ReturnDigestLog (NEW)│  │
│  │ TelemetryLog                              | NextCausalId (NEW)   │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  SCHEDULES (Multirate Execution)                                        │
│  ├── Hourly Tick:                                                       │
│  │   1. NpcRoutineSystem (moves NPCs per schedule)                      │
│  │   2. PlayerActionSystem (drains input, resolves actions)             │
│  │   3. ProductionSystem (labor produces goods)                         │
│  │   4. PhysiologySystem (metabolic decay)                              │
│  │   5. NpcGoalSystem (evaluates active goals)                          │
│  │                                                                      │
│  ├── Daily Tick (every 24 ticks):                                       │
│  │   1. EpisodicMemoryConsolidationSystem (prunes/compacts memories)    │
│  │   2. SocialVectorProgressionSystem (advances active vectors)         │
│  │   3. DemographicsAgingSystem                                         │
│  │   4. TelemetryEmissionSystem                                         │
│  │                                                                      │
│  ├── Weekly Tick (every 7 days / 168 ticks):                            │
│  │   1. MarketPriceSystem (recalculates supply/demand curves)           │
│  │   2. NarrativeGossipSystem (one-hop co-located story exchange)       │
│  │   3. HouseholdConsumptionSystem                                      │
│  │   4. RelationalPassiveNormalizationSystem (gentle baseline drift)    │
│  │                                                                      │
│  └── Monthly Tick (every 30 days / 720 ticks):                          │
│      1. TransformationCheckSystem (evaluates Scholar Stage 2)           │
│      2. MacroAbsenceVectorResolver (instantaneous multi-week fast-skip) │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Authoritative State Ownership & Data Model Discipline

| Component / Resource | Owner | Auth / Deriv | Persisted | Bounds | Why Required / Product Mapping |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `RelationalLedger` | NPC Entity | Auth | YES | Max 15 bonds (60 B) | AC-201: Qualitative relational divergence |
| `EpisodicMemory` | NPC Entity | Auth | YES | Max 18 records (540 B)| AC-202: Permanent turning-point recall |
| `EpistemicState` | NPC + Player | Auth | YES | Max 32 nodes (256 B) | AC-204: Asymmetric knowledge leverage |
| `SocialVectorRegistry` | Resource | Auth | YES | Max 16 vectors (1 KB)| AC-206, AC-207: Delayed consequence & absence |
| `ReturnDigestLog` | Resource | Auth | YES | Max 8 entries (512 B)| AC-207, AC-208: Absence return experience |
| `DocumentRegistry` | Resource | Auth | YES | Max 32 items (2 KB) | AC-205: Inscription & documentary authority |
| `EventRing` | Resource | Auth | YES | Max 1,024 events | Causal auditing & deterministic replay |
| `TelemetryLog` | Resource | Deriv | NO | Max 50,000 events | Session diagnostics & kill-test validation |

---

## 6. Relationship Architecture: The Triad Bond

### Alternative Analysis & Selection
- **Option A: 4D Float Vector `(Affection, Trust, Obligation, Deference)`**  
  *Pros:* Theoretical nuance.  
  *Cons:* Rejected in Product Review as premature architecture. Deference is redundant with social status; tuning four floats creates muddy, unpredictable behavior.
- **Option B: Event-Derived Dynamic Relationship**  
  *Pros:* No stored numbers; everything computed on the fly from memory.  
  *Cons:* $O(K)$ evaluation per query where $K$ is memory count; unstable baseline drift; breaks deterministic thresholding.
- **Option C (Selected): The Compact Triad Bond + Behavioral Mode Enum**  
  *Pros:* 4 bytes per pair. Directly maps to the three observable player forces: Warmth, Reliability, and Debt. $O(1)$ lookup.

### Data Structure
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalBond {
    pub sentiment: i8,    // -100 to +100 (Warmth vs. Hostility)
    pub trust: i8,        // -100 to +100 (Reliability vs. Suspicion)
    pub obligation: i16,  // -1000 to +1000 (Positive = NPC owes target; Negative = target owes NPC)
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalLedger {
    /// CitizenId.0 -> RelationalBond
    pub bonds: HashMap<u64, RelationalBond>,
}
```

### Behavioral Mode Deduction
Instead of hardcoding complex decision trees across all systems, the relationship architecture evaluates a clean, derived `BehavioralMode`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehavioralMode {
    DevotedAlly,        // High Sentiment (>40), High Trust (>40)
    AffectionateRefusal,// High Sentiment (>30), Low Trust (<-20) -> loves you, but won't lend tools/secrets
    GrudgingDebtor,     // Negative Sentiment (<-20), High Obligation (>50) -> hates you, but must help
    WaryConsultant,     // Neutral Sentiment (-20..20), High Trust (>50) -> professional scholar peer
    HardenedEnemy,      // Hostile Sentiment (<-40), Low Trust (<-30), No Debt
}

impl RelationalBond {
    pub fn mode(&self) -> BehavioralMode {
        if self.obligation >= 50 && self.sentiment < -20 {
            BehavioralMode::GrudgingDebtor
        } else if self.sentiment > 30 && self.trust < -20 {
            BehavioralMode::AffectionateRefusal
        } else if self.sentiment.abs() <= 20 && self.trust >= 50 {
            BehavioralMode::WaryConsultant
        } else if self.sentiment > 40 && self.trust > 40 {
            BehavioralMode::DevotedAlly
        } else if self.sentiment < -40 && self.trust < -30 {
            BehavioralMode::HardenedEnemy
        } else {
            BehavioralMode::WaryConsultant // Default neutral baseline
        }
    }
}
```
*Complexity:* 15 NPCs $\times$ 15 targets = 225 bonds total = 900 bytes of RAM.

---

## 7. Memory Architecture: Bounded Episodic Records

### Memory Record Design
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodicRecord {
    pub id: u64,
    pub tick: u64,
    pub actor: CitizenId,
    pub target: Option<CitizenId>,
    pub tag: MemoryTag,
    pub delta_sentiment: i8,
    pub delta_trust: i8,
    pub delta_obligation: i16,
    pub is_permanent: bool, // True for turning points: life-saving, severe betrayal, contract
    pub narrative_token: u16, // Maps to authored dialogue template in content definitions
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodicMemory {
    /// Rolling window of transient everyday events (max 12)
    pub transient: VecDeque<EpisodicRecord>,
    /// Permanent turning-point memories (max 6)
    pub anchors: Vec<EpisodicRecord>,
}
```

### Compaction & Decay System
- Transient events decay: every weekly schedule, transient events older than 14 days are popped from the queue using discrete integer half-life decay (`NC-08`).
- Anchors **never decay**. If an NPC acquires a 7th anchor (extremely rare in a 90-day slice), the anchor with the smallest absolute impact is converted to a transient record.
- Memory size per entity is strictly capped at $12 + 6 = 18$ records.

### Two-Stage Memory Retrieval & Dual-Stream Admission Control (`NC-31`, `NC-72`)
Gleaned from SimLab Phase 4 empirical testing (`EXP-AB-01-COMP`), naive tri-factor memory scoring causes an 81.2% retrieval collapse due to uncalibrated static importance. Godseed implements:
1. **Dual-Stream Isolation (`NC-72`):** Abstract disposition (`BehavioralMode`) and factual evidence (`EpisodicMemory`) are kept in separate ECS components so reflections never crowd out historical deeds.
2. **Relevance Dominance Scoring (`NC-31`):** During dialogue queries, candidate memories are filtered by topic domain first, then ranked by:
   $$\text{Score}(m) = 0.7 \cdot \text{Relevance}(m) + 0.3 \cdot \text{Recency}(m)$$
   where relevance mandates $\beta \ge 2\alpha$, completely eliminating distractor noise.

### One-Hop Narrative Gossip (Information-Partitioned `NC-56`)
During the weekly gossip schedule, socializing NPCs exchange stories only when true information asymmetry exists:
- Citizen A checks if Citizen B already knows the candidate knowledge node or anchor (`!listener.epistemic_state.contains(id)`).
- If novel, Citizen B adds a second-hand transient record:
```rust
EpisodicRecord {
    actor: CitizenId::PLAYER,
    tag: MemoryTag::HeardGossipAbout(CitizenId::PLAYER),
    delta_sentiment: anchor.delta_sentiment / 2,
    delta_trust: anchor.delta_trust / 2,
    is_permanent: false,
    narrative_token: anchor.narrative_token,
}
```
This guarantees AC-203 (one-hop gossip propagation) with zero redundant ECS gossip cycles (`NC-56`).

---

## 8. Knowledge Architecture: The Tripartite Model

### Content vs. State Separation
Knowledge definitions reside in static content; possession resides on entities.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeDomain {
    ObservationInsight, // e.g. Crop Blight Signs, Timber Stress, Herb Habitats
    SecretTruth,        // e.g. Voss's Exiled Son, Delia's Debt, Founding Flood
    DocumentedRecord,   // e.g. Ancient Land Charter, Signed Debt Note
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeDefinition {
    pub id: u16,
    pub domain: KnowledgeDomain,
    pub title: String,
    pub description: String,
    pub social_fallout_mode: Option<BehavioralMode>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EpistemicState {
    /// KnowledgeId -> AcquiredTick
    pub known: HashMap<u16, u64>,
}
```

### Epistemic Gating Rules
1. **Dialogue Gating:** An NPC will not display dialogue choices regarding a `SecretTruth` unless both the player and the NPC have that `KnowledgeId` in their `EpistemicState`.
2. **Action Gating:** The `Diagnose` action at North Fields fails unless the player possesses `KnowledgeDomain::ObservationInsight(CROP_BLIGHT)`.
3. **Disclosure Consequences:** Using `TalkTopic::ShareKnowledge(id)` transfers the knowledge to the NPC and triggers an immediate relationship update if the knowledge carries social fallout.

---

## 9. Causal Lineage & Event Infrastructure

To make delayed consequences verifiable and debuggable without storing an infinite graph, Godseed VS2 uses a **Lightweight Causal Pointer**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CausalPointer {
    pub root_event_id: u64,     // The player action that started this chain
    pub parent_event_id: u64,   // The intermediate event
    pub sequence_step: u8,      // Step count (1, 2, 3...)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEvent {
    pub event_id: u64,
    pub tick: u64,
    pub actor: CitizenId,
    pub causal: Option<CausalPointer>,
    pub payload: EventPayload,
}
```

When an event triggers a delayed consequence, the resulting event inherits `root_event_id`. Telemetry and integration tests can audit the complete causal ancestry in $O(1)$ time by querying `root_event_id`.

---

## 10. Delayed Consequence & Absence Architecture

### The Social Vector Registry
Delayed consequences and absence progressions are managed through the authoritative `SocialVectorRegistry` resource:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VectorStage {
    Active,     // Initial ripple occurring; forewarning cues in dialogue
    Escalated,  // Tension peaking; secondary NPCs affected
    Matured,    // Consequence finalized; structural change committed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialVector {
    pub id: u32,
    pub causal_root: u64,
    pub stage: VectorStage,
    pub target_citizen_a: CitizenId,
    pub target_citizen_b: CitizenId,
    pub trigger_tick: u64,
    pub vector_type: VectorType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VectorType {
    MerchantCreditSqueeze { creditor: CitizenId, debtor: CitizenId },
    FraternalLaborStrain  { elder: CitizenId, junior: CitizenId },
    BlightFamineDepletion { field_loc: LocationId },
    ArchiveSecretTension  { guardian: CitizenId },
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct SocialVectorRegistry {
    pub vectors: Vec<SocialVector>,
}
```

### The Progression Pipeline (Three-Trigger Protocol `NC-63`, `NC-65`)
Following SimLab Phase 4's three-trigger escalation rule, vectors never advance based solely on a timer. Advancement requires convergence of:
1. **Temporal Latency Trigger:** `current_tick >= vector.min_tick` (minimum gestation period).
2. **Practical Degradation Trigger:** Concrete settlement reality check (e.g. grain stock $< 20$, unpaid debt $\ge$ threshold).
3. **Relational Threshold Trigger:** Relational mode shifted to `GrudgingDebtor` or `HardenedEnemy`, or trust broken below $-20$.

If the player proactively remedies the practical condition before maturity, the vector defuses or diverts to a peaceful resolution.

### Execution Modes
1. **Normal Play Execution:** Every daily tick, `SocialVectorProgressionSystem` evaluates the three-trigger conditions for active vectors. When all three converge, the vector advances to its next stage, mutating NPC finances, schedules, or relationship bonds.
2. **Absence Execution (Fast-Forward Mode):** When the player skips 30 days (`Wait(720)`), the `MacroAbsenceVectorResolver`:
   - Fast-forwards commodity consumption and production in bulk blocks.
   - Evaluates all pending `SocialVector` milestones deterministically.
   - Generates an `EpistemicReturnDigest` entry in `ReturnDigestLog` for every matured vector.
3. **The Return Experience:** Upon return, the player's first interactions with relevant NPCs trigger dialogue nodes stored in `ReturnDigestLog` (e.g., *"Tomas Birch looks exhausted: 'Runn left for the forge while you were away...'"*).

---

## 11. NPC Decision Integration & Goals

VS2 does not use complex utility planning or heavy neural networks. NPC goals evaluate deterministic priority rules weighted by the `RelationalBond` with the requester:

```rust
pub fn evaluate_assistance_request(
    npc: &CitizenMeta,
    bond: &RelationalBond,
    request_type: AssistanceType,
) -> bool {
    match bond.mode() {
        BehavioralMode::DevotedAlly => true,
        BehavioralMode::HardenedEnemy => false,
        BehavioralMode::GrudgingDebtor => {
            // Must assist if obligation exceeds request cost
            bond.obligation >= request_type.cost()
        },
        BehavioralMode::AffectionateRefusal => {
            // Will provide social comfort or food, but refuses tools, coin, or secrets
            matches!(request_type, AssistanceType::ShareMeal | AssistanceType::CasualChat)
        },
        BehavioralMode::WaryConsultant => {
            // Only assists with professional/scholarly requests
            matches!(request_type, AssistanceType::DiscussArchive | AssistanceType::InspectSpecimen)
        },
    }
}
```
This is fully inspectable, 100% deterministic, and perfectly testable.

---

## 12. Scholar Progression Architecture (Stage 2)

The Scholar trajectory advances from empirical observation to documentary arbitration:

```text
Scholar Stage 1: The Inscriber
  - Capability: Inscription (Level 1)
  - Actions: Inscribe(Observation), StudyArchive
  - Social Status: "The one who records notes"

Scholar Stage 2: The Settlement Chronicler
  - Capability: Inscription (Level 2), Diagnosis
  - Actions: Diagnose(LocationId), DraftDocument(DocumentType)
  - Social Status: "Documentary Authority" (can legally arbitrate disputes)
```

### Document Representation
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InscribedDocument {
    pub id: u32,
    pub doc_type: DocumentType,
    pub drafter: CitizenId,
    pub signers: Vec<CitizenId>,
    pub binding_tick: u64,
    pub related_vector_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    DebtReliefCharter { creditor: CitizenId, debtor: CitizenId, terms: u32 },
    HarvestDiagnosisReport { location: LocationId, finding: u16 },
    FoundingArchiveTranslation { secret_id: u16 },
}
```
Documents reside as portable physical items in `Inventory`. Presenting an `InscribedDocument` to an NPC resolves active vectors in `SocialVectorRegistry`.

---

## 13. Persistence Evolution & Migration

### Schema Versioning: `GODSEED1` $\rightarrow$ `GODSEED2`
VS2 introduces new components (`EpistemicState`, `EpisodicMemory`, `RelationalLedger`) and new resources (`SocialVectorRegistry`, `ReturnDigestLog`).

```rust
pub const MAGIC_V1: &[u8; 8] = b"GODSEED1";
pub const MAGIC_V2: &[u8; 8] = b"GODSEED2";

pub fn migrate_v1_to_v2(v1_snapshot: V1Snapshot) -> V2Snapshot {
    // 1. Map scalar toward_player to RelationalBond(sentiment = val, trust = 0, obligation = 0)
    // 2. Initialize default EpistemicState from old KnowledgeInventory
    // 3. Initialize empty EpisodicMemory withMetPlayer anchor
    // 4. Initialize empty SocialVectorRegistry
    // 5. Emit migration audit event
}
```
If a `GODSEED1` save is loaded, the engine executes `migrate_v1_to_v2`, verifies the CRC32 digest, and writes a pristine `GODSEED2` file upon subsequent save. Existing saves are never silently corrupted.

---

## 14. Determinism Protocol

To guarantee that any identical sequence of inputs produces the exact same world state across platforms:
1. **No System Clocks:** Simulation time is exclusively driven by `SimClock.tick`.
2. **Deterministic PRNG:** Random choices use a seeded PCG32 generator stored in a simulation resource.
3. **Sorted ECS Iteration:** When systems iterate over queries involving multiple entities, entities are sorted by `CitizenId` before applying mutations.
4. **Deterministic Hash Verification:** FNV-1a state hasher serializes entities in strict ID order.

---

## 15. Telemetry & Causal Diagnostics

VS2 introduces three high-value telemetry events:
1. `TelemetryEvent::RelationalModeShift { actor, target, old_mode, new_mode, causal_event_id }`
2. `TelemetryEvent::VectorMatured { vector_id, outcome, causal_root_id }`
3. `TelemetryEvent::EpistemicDisclosure { speaker, listener, knowledge_id }`

These events allow headless test harnesses to verify complete cause-and-effect chains without inspecting internal memory dumps.

---

## 16. Developer & Debugging Interfaces

Accessible via CLI debug commands in debug builds:
- `debug_rel <npc_id>`: Prints the complete 3D relational bond and current `BehavioralMode` with the player and other NPCs.
- `debug_mem <npc_id>`: Lists all 12 transient and 6 permanent episodic memories with causal IDs.
- `debug_vectors`: Displays all active, escalated, and matured social vectors in `SocialVectorRegistry`.
- `debug_digest`: Dumps the current `ReturnDigestLog`.

---

## 17. Performance Bounds & Algorithmic Complexity

- **Tick Throughput Target:** $\ge 250,000$ ticks/second headless (comfortably exceeding the 60 Hz interactive requirement).
- **RAM Target:** $\le 30$ MB resident memory.
- **Complexity Analysis:**
  - `RelationalBond` lookup: $O(1)$ hash map lookup (max 15 entries per entity).
  - Memory consolidation: $O(1)$ (bounded at 18 elements).
  - Gossip propagation: $O(N)$ for co-located citizens at the same spatial node (max 5 citizens per location).
  - Social vector tick: $O(V)$ where $V \le 16$.
  - Entire frame compute cost is strictly $O(N)$ with $N=15$.

---

## 18. Invariant Suite (Machine-Checkable)

The machine-checkable invariant suite is expanded from INV-1..3 to INV-1..7:
- **INV-1 (Population Integrity):** Total alive citizens $\in [1, 16]$; citizen IDs are unique.
- **INV-2 (Currency Conservation):** Sum of personal coins + settlement treasury $\ge 0$.
- **INV-3 (Metabolic Bounds):** Satiety, rest, health $\in [0, 100]$.
- **INV-4 (Relational Symmetry & Self-Exclusion):** No citizen holds a `RelationalBond` with themselves (`bonds.get(self.id).is_none()`).
- **INV-5 (Memory Bounds):** For all NPCs, `transient.len() <= 12` and `anchors.len() <= 6`.
- **INV-6 (Vector Integrity):** Every active `SocialVector` references valid alive `CitizenId`s and a registered `causal_root`.
- **INV-7 (Epistemic Validity):** All IDs in `EpistemicState.known` resolve to existing definitions in `ContentDefinitions`.

---

## 19. Failure Containment & Defensive Design

1. **Dangling Knowledge Node:** If an NPC attempts to share a `KnowledgeId` that does not exist in content, the system logs a `SimError` and drops the action rather than panicking.
2. **Missing Vector Target:** If an NPC involved in a `SocialVector` dies of starvation during a time skip, the vector terminates with status `VectorStage::AbortedTargetDeceased` and records a return digest entry explaining the tragedy.
3. **Save File Deserialization Failure:** CRC32 mismatch or truncated headers halt loading with a clean error message: *"Save file corrupted or invalid version"*.

---

## 20. Implementation Slicing

To maintain a compiling, runnable vertical slice at every step, implementation is structured into five sequential slices:

```text
┌────────────────────────────────────────────────────────────────────────┐
│                   VS2 IMPLEMENTATION SEQUENCE                         │
├─────────┬───────────────────────────────┬──────────────────────────────┤
│ Slice A │ Relational Bond & Episodic    │ AC-201 (Relational Diverg),  │
│         │ Memory Substrate              │ AC-202 (Episodic Recall)     │
├─────────┼───────────────────────────────┼──────────────────────────────┤
│ Slice B │ One-Hop Narrative Gossip &    │ AC-203 (One-Hop Gossip),     │
│         │ Asymmetric Knowledge          │ AC-204 (Knowledge Leverage)  │
├─────────┼───────────────────────────────┼──────────────────────────────┤
│ Slice C │ Social Vectors & Delayed      │ AC-206 (Delayed Consequence),│
│         │ Consequence Pipeline          │ AC-207 (Absence Continuation)│
├─────────┼───────────────────────────────┼──────────────────────────────┤
│ Slice D │ Scholar Stage 2 (Diagnosis    │ AC-205 (Epistemic Agency &   │
│         │ & Document Drafting)          │ Inscribed Documents)         │
├─────────┼───────────────────────────────┼──────────────────────────────┤
│ Slice E │ Return Digest & Playtest      │ AC-208 (Kill Test & Human    │
│         │ Telemetry Instrument          │ Playtest Candidate)          │
└─────────┴───────────────────────────────┴──────────────────────────────┘
```

---

## 21. Test Architecture

The automated verification framework consists of five testing layers:
1. **Unit Tests:** Component bounds, `BehavioralMode` deduction, episodic memory decay rules.
2. **Integration Scenarios:**
   - `test_ac201_relational_divergence`: Compares Wren's vs. Mira's responses to identical player requests.
   - `test_ac202_episodic_recall`: Advances time 14 days and checks dialogue response.
   - `test_ac203_one_hop_gossip`: Verifies event propagation between co-located NPCs.
   - `test_ac204_asymmetric_knowledge`: Verifies decision change upon knowledge disclosure.
   - `test_ac205_scholar_stage2_inscription`: Verifies document creation and dispute resolution.
   - `test_ac206_delayed_consequence`: Verifies Week 1 action $\rightarrow$ Week 3 consequence.
   - `test_ac207_absence_continuation`: Verifies 30-day off-screen vector maturation.
3. **Persistence Round-Trip Tests:** `GODSEED1` migration and `GODSEED2` exact deserialization.
4. **90-Day Continuous Soak Test:** Verifies INV-1 through INV-7 across 2,160 ticks with zero invariant violations.
5. **Replay & Determinism Suite:** Bit-for-bit hash verification from seed.

---

## 22. Human Playtest Support Tooling

To satisfy AC-208 and support the human evaluation gate, the architecture includes the **Playtest Session Flight Recorder**:
- Automatically records the playthrough seed, command history, and causal consequence tree into `saves/playtest_<timestamp>.session`.
- Generates a human-readable **Causal Narrative Audit**:
  ```text
  [Day 3, 08:00] Player assisted Tomas with timber felling. (Root #104)
  [Day 11, 14:00] Vector #12 (FraternalLaborStrain) matured: Runn apprenticed to Wren.
  [Day 30, 09:00] Player returned: Mira reported Runn's career shift.
  ```
This enables evaluators to instantly cross-reference human interview retellings with factual simulation causality.

---

## 23. Migration Matrix

```text
EXISTING VS1 COMPONENT        VS2 STATUS      ACTION
---------------------------------------------------------------------------------
CitizenMeta                   UNCHANGED       Keep as identity marker
Demographics                  UNCHANGED       Keep for age/health
PhysicalNeeds                 UNCHANGED       Keep for metabolic substrate
Inventory                     EXTENDED        Add InscribedDocument support
SettlementRef                 UNCHANGED       Spatial containment
HouseholdRef                  UNCHANGED       Domestic containment
Kinship                       UNCHANGED       Family links
NpcMemory                     REPLACED        Replaced by EpisodicMemory
NpcSchedule                   EXTENDED        Dynamic schedule slot reassignment
NpcGoals                      EXTENDED        Relational mode goal weighting
Disposition                   REPLACED        Replaced by RelationalLedger
PlayerMarker                  UNCHANGED       Marker component
CapabilitySet                 EXTENDED        Add Inscription Level 2
TransformationState           EXTENDED        Add Scholar Stage 2 milestones
KnowledgeInventory            REPLACED        Replaced by EpistemicState
PlayerInputBuffer             EXTENDED        Add Diagnose & InscribeDoc actions
SimClock                      UNCHANGED       Simulation time
WorldMap                      UNCHANGED       11 spatial nodes
SettlementDirectory           UNCHANGED       Commodities & market prices
HouseholdDirectory            UNCHANGED       Rations & housing
EventRing                     EXTENDED        Add CausalPointer to SimEvent
ContentDefinitions            EXTENDED        Add KnowledgeDef & DocumentDef
TelemetryLog                  EXTENDED        Add Causal & Relational events
```

---

## 24. Technical Risks & Mitigations

| Risk | Severity | Mitigation |
| :--- | :--- | :--- |
| **Episodic memory explosion in soak tests** | High | Hard limits enforced by ECS component invariant: 12 transient + 6 anchors. Excess dropped deterministically. |
| **Combinatorial relational chaos** | Medium | Relationships governed by derived `BehavioralMode` enum with standard default fallback. |
| **Fast-forward absence desynchronization** | High | Macro-tick resolver evaluates vectors using deterministic milestones rather than approximating physics. |
| **Save migration data loss** | Medium | Explicit migration function converts `GODSEED1` scalars into default `GODSEED2` triad bonds without data loss. |

---

## 25. Architectural Decision Records (ADRs)

### ADR-001: The Compact Triad Relationship Bond
- **Decision:** Use a 3-field integer struct `(sentiment: i8, trust: i8, obligation: i16)` mapped to a derived `BehavioralMode` enum, rather than a 4D float vector or dynamic event evaluator.
- **Product Requirement:** AC-201 (Qualitative Relational Divergence).
- **Options Considered:** 4D Float Vector; Dynamic Memory Evaluator; Triad Bond.
- **Tradeoffs:** Gives up granular floating-point nuance in exchange for $O(1)$ speed, trivial serialization, and clear behavioral boundaries.
- **Reversibility:** High; the derived enum abstracts the underlying struct from systems.

### ADR-002: Hard-Bounded Episodic Memory with Anchor Retention
- **Decision:** Split NPC memory into 12 transient FIFO slots with time decay and 6 permanent anchor slots for momentous events.
- **Product Requirement:** AC-202 (Episodic Narrative Recall).
- **Options Considered:** Unbounded event list; Rolling 20 FIFO (VS1); Fixed Anchor/Transient Split.
- **Tradeoffs:** Caps total remembered turning points per NPC at 6, which is more than sufficient for a 90-day slice while guaranteeing zero memory leakage.
- **Reversibility:** Medium; requires persistence schema support.

### ADR-003: Tripartite Knowledge Representation
- **Decision:** Divide knowledge definitions into ObservationInsight, SecretTruth, and DocumentedRecord, stored in an entity `EpistemicState` hashmap.
- **Product Requirement:** AC-204 (Asymmetric Knowledge Leverage).
- **Options Considered:** Flat boolean list (VS1); Full Epistemology Graph; Tripartite Categorical Map.
- **Tradeoffs:** Avoids graph traversal while cleanly gating dialogue, actions, and social danger.
- **Reversibility:** High.

### ADR-004: Social Vectors for Delayed Consequence & Absence Continuation
- **Decision:** Use an authoritative `SocialVectorRegistry` resource to track unresolved cause-and-effect chains and advance them during absence.
- **Product Requirement:** AC-206 (Delayed Consequence), AC-207 (Absence Continuation).
- **Options Considered:** Hand-scripted quest timers; Full continuous micro-tick simulation during absence; Social Vector State Machine.
- **Tradeoffs:** Micro-ticks during a 60-day skip would cause UI lag. Social vectors evaluate deterministically in constant time.
- **Reversibility:** Medium.

### ADR-005: Schema Version Bump to `GODSEED2` with Upward Migration
- **Decision:** Advance save format magic from `GODSEED1` to `GODSEED2`, providing an explicit migration routine that converts legacy scalar dispositions into baseline triad bonds.
- **Product Requirement:** Persistence integrity and backward compatibility.
- **Options Considered:** Breaking compatibility (wipe old saves); In-place hacky deserialization; Explicit versioned migration.
- **Tradeoffs:** Requires writing a dedicated migration adapter, but preserves developer and tester save files.
- **Reversibility:** Low (governs binary wire format).

### ADR-006: Integration of SimLab Empirical Simulation Optimizations
- **Decision:** Adopt SimLab Phase 4 empirical findings: Two-Stage Relevance-Dominant Memory Retrieval (`NC-31`), Dual-Stream Memory Partitioning (`NC-72`), Asymmetric Information Transmission (`NC-56`), Discrete Exponential Normalization (`NC-08`), and Three-Trigger Vector Progression (`NC-63`).
- **Product Requirement:** AC-201, AC-202, AC-203, AC-204, AC-206.
- **Optimization Dossier Reference:** [GODSEED_VS2_SIMLAB_OPTIMIZATIONS.md](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/GODSEED_VS2_SIMLAB_OPTIMIZATIONS.md)
- **Tradeoffs:** Requires slightly stricter filtering in gossip and memory pipelines, but eliminates memory retrieval collapse (+431% MRR improvement demonstrated in SimLab `EXP-AB-01`), removes redundant gossip iterations, and grounds delayed consequences in physical settlement state.
- **Reversibility:** High; implemented entirely inside system queries without affecting component schemas.

