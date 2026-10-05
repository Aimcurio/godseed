# Godseed VS2 Social Authority Remediation Report

## Scope

Target branch: `vs2-implementation`

Audited predecessor: `3312a814ce3cdcbe4907b190c64854fc706e9f01`

Required workflow state: `REMEDIATE`

Merge status: not merged. Draft PR #1 must remain draft. AC-208 remains deferred.

## Canonical Authority Contract

For V2 runtime gameplay, all historical social relationship decisions are derived from `RelationalLedger`, `EpisodicMemory`, and `EpistemicState`.

No normal gameplay system may read legacy relationship or memory state to decide behavior.

Legacy structures may exist only for compatibility with old save data and migration into V2 authoritative state.

## Remediation Summary

- Removed the live global `RelationshipLedger` resource from new V2 simulations and loaded V2 runtime worlds.
- Rewired inspection, talk responses, offers, teaching gates, transformation recognition, felling assistance, arbitration effects, CLI relationship display, and gossip opinion propagation to use entity-local `RelationalLedger` bonds.
- Stopped scheduling legacy `npc_memory_system` and legacy relationship decay.
- Stopped spawning fresh V2 NPCs with `NpcMemory`.
- Preserved V1 save compatibility by keeping legacy snapshot structs and migrating V1 relationship values into V2 `RelationalLedger`.
- Added a regression test proving the legacy `RelationshipLedger` is not installed as a V2 runtime gameplay resource.
- Updated tests that previously asserted social state through the legacy ledger to assert through canonical VS2 bonds.
- Added `README.md`, `rust-toolchain.toml`, and `.github/workflows/ci.yml`.

## Acceptance Criteria

| Criterion | Disposition | Evidence |
|---|---:|---|
| AC-201 qualitative relational divergence | PASS | `RelationalLedger` drives dialogue, offer gates, relationship deltas, felling bonds, arbitration bonds, and tests. |
| AC-202 permanent turning-point recall | PASS | `EpisodicMemory` anchors survive save/load and drive felling recall. |
| AC-203 one-hop gossip propagation | PASS | Gossip system propagates opinion and knowledge through VS2 state. |
| AC-204 asymmetric knowledge leverage | PASS | `EpistemicState` tests cover knowledge sharing, corroboration, and document prerequisites. |
| AC-205 inscription/documentary authority | PASS | `DocumentRegistry` and arbitration tests cover document creation and applicability. |
| AC-206 delayed consequence pipeline | PASS | `PendingConsequenceRegistry` tests cover delayed consequence maturation and guard behavior. |
| AC-207 absence continuation/return digest | PASS | `ReturnDigestLog` and macro-absence tests cover absence consequences and return digest persistence. |
| AC-208 human retelling/qualitative playtest | DEFERRED | Human evaluation remains outside this technical remediation. |

## Authority Verdict

The V2 runtime social model is now canonical for gameplay relationship decisions. Legacy social structures remain in code only for old-format deserialization and compatibility.

## Risks

### High

None known after validation.

### Medium

- V2 snapshot structs still carry an inert `relationships` field for compatibility shape. It is intentionally empty for V2-built snapshots, but a future save-format cleanup should remove or version-gate it.
- `Disposition` still stores static teaching metadata and legacy dynamic fields. Runtime gameplay no longer uses dynamic relationship output, but future refactor should split static teaching configuration from the legacy dynamic structure.

### Low

- `systems::relationship` and `systems::npc_memory` remain as orphaned legacy modules. They are not scheduled in V2 runtime. A later refactor should remove or archive them once public API compatibility is settled.

## Required Fixes

None remaining for the current remediation slice.

## Optional Improvements

- Introduce a dedicated `TeachingProfile` component to remove the last non-authoritative teaching metadata from `Disposition`.
- Create `SimulationSnapshotV3` without inert legacy fields after downstream compatibility requirements are reviewed.
- Add a static lint or architecture test that scans system signatures for legacy social authority resources.

## Commands Required For Requalification

```powershell
git branch --show-current
git rev-parse HEAD
git rev-parse HEAD^{tree}
git status --porcelain
git rev-parse origin/vs2-implementation
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p godseed_cli --bin soak_test
```

## Test Census

Post-remediation validation passed on Windows with Rust/Cargo `1.97.1`.

| Command | Result |
|---|---:|
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo test --workspace` | PASS |
| `cargo run -p godseed_cli --bin soak_test` | PASS |

`cargo test --workspace` census:

| Test target | Passed | Failed |
|---|---:|---:|
| `src/main.rs` unit harness | 0 | 0 |
| `src/bin/soak_test.rs` unit harness | 0 | 0 |
| `src/lib.rs` unit harness | 0 | 0 |
| `ac_embodiment_and_population.rs` | 4 | 0 |
| `ac_persistence_and_determinism.rs` | 6 | 0 |
| `ac_progression_and_transformation.rs` | 2 | 0 |
| `ac_social_and_economy.rs` | 3 | 0 |
| `adversarial_and_integrity.rs` | 7 | 0 |
| `epistemic_gossip.rs` | 1 | 0 |
| `macro_absence_proof_c.rs` | 1 | 0 |
| `remediation_persistence_and_lifecycle.rs` | 10 | 0 |
| `scholar_stage2_arbitration.rs` | 1 | 0 |
| `thin_causal_slice.rs` | 1 | 0 |
| `godseed_core` doctests | 0 | 0 |

Total executable Rust tests: 36 passed, 0 failed.

Soak census:

- Total ticks: 10,000.
- Simulated duration: 416.7 days.
- Checkpoints: 20/20 invariant-pass.
- Final soak hash: `169acdae9e3adf95`.
- Throughput: approximately 27,592 ticks/sec.
- Evidence files updated: `evidence/GODSEED_VS2_SOAK_REPORT.json`, `evidence/soak_test_report.json`.
