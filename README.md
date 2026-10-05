# Godseed

Godseed is a Rust simulation prototype for a persistent settlement slice in Thornveil.

## Current Branch

`vs2-implementation` contains the Vertical Slice 2 social authority work. The canonical V2 runtime social model is:

- `RelationalLedger`
- `EpisodicMemory`
- `EpistemicState`
- `DocumentRegistry`
- `PendingConsequenceRegistry`
- `ReturnDigestLog`

Legacy `RelationshipLedger`, `NpcMemory`, and dynamic `Disposition` data are compatibility and migration concerns only. Normal V2 gameplay must not use them as independent sources of social truth.

## Build And Test

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p godseed_cli --bin soak_test
```

## Repository Layout

- `crates/godseed_core`: ECS simulation, persistence, systems, invariants, and tests.
- `crates/godseed_cli`: terminal interface and soak-test binary.
- `evidence`: generated telemetry and evaluation artifacts.
- `GODSEED_VS2_REQUIREMENT_ARCHITECTURE_TRACEABILITY.md`: VS2 acceptance traceability.

## Acceptance Status

AC-201 through AC-207 are covered by deterministic Rust tests and soak validation. AC-208 is a human qualitative playtest criterion and remains deferred.
