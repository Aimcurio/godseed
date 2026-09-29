# GODSEED PROJECT DOSSIER

**Vertical Slice 1: Thornveil Settlement**  
**Repository**: `C:\Users\15103\.gemini\antigravity\scratch\godseed`  
**Current Gate**: `PR_READY_CANDIDATE`  
**Engine**: Bevy ECS 0.15 (Headless Engine)  
**Binary Targets**: `godseed` (CLI Interactive RPG), `soak_test` (Benchmark Utility)  
**Date**: 2026-09-29  

---

## 1. Executive Summary

**Godseed** is a simulation-driven, persistent-world role-playing game where a physically embodied player arrives as an outsider in an autonomous, living settlement. Unlike conventional RPGs with scripted quest hubs and static vendor NPCs, every inhabitant in Godseed lives on a 24-hour routine, manages personal needs and finances, engages in trade, and forms lasting social memories.

This Dossier documents the complete, evidence-backed implementation of **Vertical Slice 1 (Thornveil)**, executing the full development campaign from Product Contract through Architecture, Implementation, Multi-Persona Life Testing, Fun Gate Validation, Adversarial Verification, Performance Profiling, and Release Packaging.

### Campaign Disposition: `PR_READY_CANDIDATE`
- **Playable**: Fully functional terminal RPG interface with comprehensive command palette (`look`, `go`, `inspect`, `talk`, `buy`, `sell`, `work`, `practice`, `learn`, `inscribe`, `study`, `sleep`, `wait`, `skip`, `save`, `status`, `invariants`).
- **Verifiable**: 20 automated integration and unit tests passing with zero failures.
- **Durable**: 7 scripted personas tested across 30 simulated days; 90-day soak test completed with zero invariant violations and 100% population survival.
- **Performant**: 598,620 ticks/second headless throughput; 5.45 MB binary; 14.2 MB resident memory.

---

## 2. Requirements Traceability Matrix (AC-1 through AC-14)

| Req ID | Requirement Description | Implementing System / Component | Primary Verification Mechanism | Status |
| :--- | :--- | :--- | :--- | :---: |
| **AC-1** | Physical embodiment & survival needs (satiety, health, rest) | `PhysicalNeeds`, `physiology_system` | `test_ac1_physical_embodiment_and_needs` | **PASS** |
| **AC-2** | Settlement population (15 authored Thornveil NPCs) | `ContentDefinitions`, `spawn_thornveil_npcs` | `test_ac2_settlement_population` | **PASS** |
| **AC-3** | Autonomous NPC daily routines across 24h schedule | `NpcSchedule`, `npc_routine_system` | `test_ac3_autonomous_npc_routines` | **PASS** |
| **AC-4** | Meaningful dialogue & social relationship dynamics | `RelationshipLedger`, `player_action_system` | `test_ac4_dialogue_and_social_relationship` | **PASS** |
| **AC-5** | Economic participation (buying, selling, prices) | `SettlementDirectory`, `market_price_update_system` | `test_ac5_and_ac8_economy_and_settlement_alteration` | **PASS** |
| **AC-6** | Capability acquisition & practice (Woodcutting, Inscription, etc.) | `CapabilitySet`, `player_action_system` | `test_ac6_capability_acquisition_and_progression` | **PASS** |
| **AC-7** | Transformation path (The Inscription Path to Scholar) | `TransformationState`, `transformation_check_system` | `test_ac7_transformation_path_the_inscription_path` | **PASS** |
| **AC-8** | Settlement alteration (commodity stocks & prices shift) | `Settlement`, `production_system`, `market.rs` | `test_ac5_and_ac8_economy_and_settlement_alteration` | **PASS** |
| **AC-9** | Persistence & save/load integrity (bincode + CRC32) | `persistence.rs` (`GODSEED1` format) | `test_ac9_persistence_and_save_load_integrity` | **PASS** |
| **AC-10**| Deterministic state replay & hash verification | `replay.rs` (FNV-1a state hasher) | `test_ac10_state_determinism` | **PASS** |
| **AC-11**| Departure & return consequences across time skips | Multirate scheduler, `sim.advance()` | `test_ac11_departure_and_return_consequences` | **PASS** |
| **AC-12**| Invariant suite enforcement (INV-1, INV-2, INV-3) | `invariants.rs` (`verify_invariants`) | `test_ac12_invariants_suite`, Soak Test | **PASS** |
| **AC-13**| Gossip propagation & social memory | `NpcMemory`, `gossip_system` | `test_ac13_gossip_and_social_memory` | **PASS** |
| **AC-14**| Telemetry emission & causal decision auditing | `TelemetryLog`, `telemetry_sys.rs` | `test_ac14_telemetry_emission`, Persona Logs | **PASS** |

---

## 3. World Content Catalog

### 3.1 Thornveil Settlement Map (11 Locations)
1. **The Slanted Timber (Inn)** (Loc 1): Social hub, food, lodging, common room. Managed by Mira.
2. **Wren's Forge** (Loc 2): Metalworking and charcoal smithy. Managed by Wren.
3. **Market Square** (Loc 3): Trading plaza with shifting stalls. Managed by Delia and Harwin.
4. **North Fields** (Loc 4): Grain and rye farming plots. Worked by Oswin and Corva.
5. **South Fields** (Loc 5): Vegetable patches and flax fields. Worked by Pella, Gwen, and Nissa.
6. **Herb Garden** (Loc 6): Medicinal cultivation garden. Tended by Sera.
7. **The Well** (Loc 7): Central water source and casual gathering point.
8. **The Old Archive** (Loc 8): Ruined stone repository of forgotten knowledge. Central to Inscription path.
9. **Settlement Road** (Loc 9): Main packed-mud thoroughfare; player arrival point.
10. **Storage House** (Loc 10): Settlement granary and tool store. Managed by Bard and Elder Voss.
11. **Forest Edge** (Loc 11): Timber cutting stands. Worked by Tomas and Runn.

### 3.2 Authored Inhabitants (15 NPCs)
1. **Mira Ashbridge** (Citizen 1): Pragmatic innkeeper; teaches Cooking at high rapport.
2. **Wren Forscythe** (Citizen 2): Suspicious blacksmith; teaches Smithing at high rapport.
3. **Oswin Cley** (Citizen 3): Weathered farmer; teaches Farming.
4. **Sera Cley** (Citizen 4): Perceptive herbalist; teaches Herbalism.
5. **Elder Voss** (Citizen 5): Settlement elder and former keeper of the archive; key gatekeeper to the **Inscription Transformation Path**.
6. **Tomas Birch** (Citizen 6): Sturdy forester; teaches Woodcutting.
7. **Delia Croft** (Citizen 7): Sharp merchant; teaches Trading.
8. **Harwin Croft** (Citizen 8): Delia's partner and ledger keeper.
9. **Pella** (Citizen 9): Observant day laborer and social hub.
10. **Aldous Minner** (Citizen 10): Quarry miner.
11. **Gwen Minner** (Citizen 11): Farm laborer.
12. **Runn** (Citizen 12): Timber laborer.
13. **Corva** (Citizen 13): Field laborer.
14. **Bard Tholl** (Citizen 14): Carpenter and artisan.
15. **Nissa Tholl** (Citizen 15): Agricultural laborer.

---

## 4. Systems Architecture

Godseed uses a multirate headless Bevy ECS architecture:
- **Tick Rate**: 1 tick = 1 in-game hour (24 ticks = 1 in-game day).
- **Daily Schedule (runs every tick)**:
  - `npc_routine_system`: Moves NPCs according to hourly routine schedules.
  - `player_action_system`: Validates and resolves player actions from input buffer.
  - `production_system`: Working NPCs and laborers deposit commodities into stockpiles.
  - `physiology_system`: Decays satiety and rest; handles starvation and recovery.
  - `npc_goal_system` & `npc_memory_system`: Forms NPC impressions from witnessed events.
  - `demographics_aging_system`: Advances chronological age and handles mortality.
  - `telemetry_system`: Emits once-daily consolidated telemetry records.
- **Periodic Schedule (runs every 7 ticks / ~3 times per day)**:
  - `market_price_update_system`: Adjusts commodity prices based on stockpile scarcity.
  - `household_consumption_system`: Inhabitants consume food supplies from settlement reserves.
  - `labor_market_system`: Distributes earned wages to working citizens.
  - `gossip_system`: Propagates social sentiment across co-located citizens.
  - `relationship_decay_system`: Gently normalizes extreme relationships toward baseline.
- **Monthly Schedule (runs every 30 ticks)**:
  - `transformation_check_system`: Evaluates requirements for Scholar initiation, inscription milestones, and title progression.

---

## 5. Evidence Archive Index

All generated evidence files reside under `evidence/`:
- `evidence/interactive_session_transcript.txt`: Complete log of executor-in-the-loop interactive play session.
- `evidence/soak_test_report.json`: Full telemetry and checkpoint hashes from 90-day soak benchmark.
- `evidence/telemetry_cooperative.jsonl`: Telemetry log for Cooperative persona life test.
- `evidence/telemetry_opportunist.jsonl`: Telemetry log for Opportunist persona life test.
- `evidence/telemetry_transformation.jsonl`: Telemetry log for Transformation persona life test.
- `evidence/telemetry_social_aggressive.jsonl`: Telemetry log for Social Aggressive persona life test.
- `evidence/telemetry_knowledge_seeker.jsonl`: Telemetry log for Knowledge Seeker persona life test.
- `evidence/telemetry_ignore_hooks.jsonl`: Telemetry log for Ignore Hooks persona life test.
- `evidence/telemetry_explorer.jsonl`: Telemetry log for Explorer persona life test.

---

## 6. Scope Boundaries & Future Roadmap

### In Scope for VS1 (Completed):
- Single settlement (Thornveil) with 15 fully authored NPCs and 11 locations.
- Complete core loop: survival needs, labor, trade, dialogue, capability progression.
- Stage 0 and Stage 1 of the Inscription Transformation Path.
- Terminal user interface with human-readable text and clean command parser.
- Deterministic simulation, save/load persistence, machine-checkable invariants.

### Explicitly Deferred to Vertical Slice 2 & 3:
- Multiple interconnected settlements and inter-settlement caravan trade.
- Procedural NPC generation and generative family lineages.
- Stages 2 and 3 of the Inscription Path (Master Inscriber, Weaver of Edicts).
- 2D/3D graphical frontends (web client, TUI curses canvas, or Bevy native window).
- Combat, crime, banditry, and military conflict systems.

---

## 7. Final Quality Gate Verdict

| Gate | Disposition | Verification Summary |
| :--- | :--- | :--- |
| **Phase 0: Product Contract** | `GODSEED_PHASE_0_PASS` | Contract frozen with 14 ACs and 6 Fun Hypotheses |
| **Phase 1: Architecture** | `GODSEED_PHASE_1_ARCHITECTURE_PASS` | Full system design, persistence boundary, and data ownership |
| **Phase 2: Greybox Slice** | `GODSEED_PHASE_2_PASS` | Complete compilation, zero compiler warnings |
| **Life Testing** | `GODSEED_LIFE_TEST_PASS` | 7 personas tested across 30 days; 90-day soak test passed |
| **Fun Gate** | `GODSEED_FUN_GATE_PASS` | All 6 Falsifiable Fun Hypotheses validated with evidence |
| **Adversarial & Integrity** | `GODSEED_ADVERSARIAL_PASS` | Bit-flip corruption rejected; invalid inputs handled safely |
| **Performance** | `GODSEED_PERFORMANCE_PASS` | 598,620 ticks/sec; 5.45 MB binary; 14.2 MB memory |
| **Final Campaign Verdict** | **`PR_READY_CANDIDATE`** | Ready for code review, PR submission, and release tagging |
