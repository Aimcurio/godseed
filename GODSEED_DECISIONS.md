# GODSEED DECISION LOG

**Repository**: `godseed`  
**Standard**: Architecture and Product Change Control Record  
**Governing Directive**: Section 28 of Godseed Master Directive  

---

## Decision Record 001: Player Bundle Splitting for Bevy ECS 0.15 Bound Limit

- **Date**: 2026-09-29
- **Status**: Implemented & Verified
- **Context**: In `bevy_ecs 0.15`, the `Bundle` trait is implemented for tuples up to arity 15. The Godseed player entity combines standard citizen biology (10 components) with player progression states (`CapabilitySet`, `TransformationState`, `KnowledgeInventory`, `PlayerInputBuffer`, `CausalAudit`, and `PlayerMarker`), totaling 16 components. Initial flat tuple spawn resulted in `error[E0277]: (...) is not a Bundle`.
- **Decision**: Spawn the entity with base citizen archetypes, followed immediately by `.insert()` of the player-specific progression bundle.
- **Impacted Systems**: [`crates/godseed_core/src/player.rs`](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/crates/godseed_core/src/player.rs)
- **Contract & Test Impact**: None. All components remain present on the player entity and queryable via ECS archetypes. Verified by `test_ac1_physical_embodiment_and_needs`.

---

## Decision Record 002: QueryData Nested Tuple Grouping for Persistence Serialization

- **Date**: 2026-09-29
- **Status**: Implemented & Verified
- **Context**: Bincode persistence serialization requires capturing every component of all living citizens (both shared and optional NPC/player states). Querying 19 distinct component types in a flat tuple exceeded Bevy's `QueryData` tuple limit of 15.
- **Decision**: Nest the query into two logical sub-tuples:
  1. Base biological and economic components: `(&CitizenMeta, &Demographics, &HouseholdRef, &SettlementRef, &OccupationProfile, &PersonalFinances, &PhysicalNeeds, &MobilityProfile, &Kinship, &CausalAudit, &Inventory)`
  2. Role-specific optionals: `(Option<&NpcMemory>, Option<&NpcSchedule>, Option<&NpcGoals>, Option<&Disposition>, Option<&PlayerMarker>, Option<&CapabilitySet>, Option<&TransformationState>, Option<&KnowledgeInventory>)`
- **Impacted Systems**: [`crates/godseed_core/src/sim.rs`](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/crates/godseed_core/src/sim.rs)
- **Contract & Test Impact**: Zero data loss. Verified by `test_ac9_persistence_and_save_load_integrity`.

---

## Decision Record 003: Agricultural Production Calibration for Field Laborers

- **Date**: 2026-09-29
- **Status**: Implemented & Verified
- **Context**: In the initial draft of `production_system`, only NPCs with explicit `OccupationType::Farmer` produced food. In Thornveil's authored population of 15, only Oswin Cley held the Farmer title, while 4 other citizens held `Laborer` titles but had schedules placing them in the North and South agricultural fields. This caused an artificial agricultural deficit over long multi-week horizons.
- **Decision**: Calibrate `production_system` so that `Laborer` citizens assigned to agricultural field locations (`LocationId(4)` and `LocationId(5)`) produce grain and vegetables (0.35 food/hour × productivity), while laborers at the forest edge produce timber.
- **Impacted Systems**: [`crates/godseed_core/src/systems/production.rs`](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/crates/godseed_core/src/systems/production.rs)
- **Contract & Test Impact**: Successfully stabilized settlement survival over 90+ days without falsifying individual schedules. Verified by 90-day soak test maintaining 15/15 living inhabitants.

---

## Decision Record 004: Multirate Scheduler Calibration for 7-Tick Periodic Cycles

- **Date**: 2026-09-29
- **Status**: Implemented & Verified
- **Context**: To maintain dynamic gameplay in terminal sessions, periodic economic ticks run every 7 in-game hours (`tick % 7 == 0`). Initial consumption logic deducted a full week's worth of rations (3.5 food units) per execution, which drained 168 hours of provisions in 7 hours.
- **Decision**: Proportionally calibrate the periodic cycle: each NPC consumes 0.15 food units per 7-hour cycle (`3.5 × 7 / 168 = 0.146`), restoring satiety smoothly. Wages are similarly scaled (`daily_wage × 7 / 24`).
- **Impacted Systems**: [`crates/godseed_core/src/systems/market.rs`](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/crates/godseed_core/src/systems/market.rs), [`crates/godseed_core/src/systems/labor.rs`](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/crates/godseed_core/src/systems/labor.rs)
- **Contract & Test Impact**: Ensures long-term economic and biological equilibrium across 30-day and 90-day simulation runs. Verified by all 7 persona life tests.

---

## Decision Record 005: Dual Initiation Paths for Inscription Transformation

- **Date**: 2026-09-29
- **Status**: Implemented & Verified
- **Context**: `TransformationPath::None` initially required knowledge of `ELDER_VOSS_SECRET` to unlock Stage 0. However, exploratory players could also directly discover the ruined Old Archive (`LocationId(8)`) and practice Inscription through autonomous experimentation.
- **Decision**: Enable dual valid initiation pathways:
  1. Traditional social path: High rapport with Elder Voss + lore disclosure.
  2. Empirical discovery path: Visiting the Old Archive and practicing the `INSCRIPTION` capability directly advances player status to **Stage 1 (Scholar)**.
- **Impacted Systems**: [`crates/godseed_core/src/systems/transformation.rs`](file:///C:/Users/15103/.gemini/antigravity/scratch/godseed/crates/godseed_core/src/systems/transformation.rs)
- **Contract & Test Impact**: Enhances player agency and rewards multiple playstyles without breaking narrative coherence. Verified by `test_ac7_transformation_path_the_inscription_path`.
