/// Godseed — Persistence (bincode + CRC32, Version 2 with V1 upward migration)

use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use crc32fast::Hasher as CrcHasher;

use crate::components::{
    CausalAudit, CapabilitySet, CitizenMeta, Demographics, Disposition,
    EpisodicMemory, EpistemicState, HouseholdRef,
    Inventory, Kinship, KnowledgeInventory, MobilityProfile, NpcGoals, NpcMemory, NpcSchedule,
    OccupationProfile, PersonalFinances, PhysicalNeeds, RelationalLedger, SettlementRef,
    TransformationState,
};
use crate::household::HouseholdDirectory;
use crate::resources::{
    DocumentRegistry, EventRing, PendingConsequenceRegistry,
    RelationshipLedger, ReputationRegistry, ReturnDigestLog,
};
use crate::settlement::SettlementDirectory;
use crate::types::SimClock;
use crate::world::WorldMap;

pub const MAGIC_V1: &[u8; 8] = b"GODSEED1";
pub const MAGIC_V2: &[u8; 8] = b"GODSEED2";
pub const FORMAT_VERSION_V1: u32 = 1;
pub const FORMAT_VERSION_V2: u32 = 2;

pub const MAGIC: &[u8; 8] = MAGIC_V2;
pub const FORMAT_VERSION: u32 = FORMAT_VERSION_V2;

// ── V1 Legacy Snapshot Types (for upward migration) ──────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CitizenSnapshotV1 {
    pub meta: CitizenMeta,
    pub demographics: Demographics,
    pub household_ref: HouseholdRef,
    pub settlement_ref: SettlementRef,
    pub occupation: OccupationProfile,
    pub finances: PersonalFinances,
    pub needs: PhysicalNeeds,
    pub mobility: MobilityProfile,
    pub kinship: Kinship,
    pub causal_audit: CausalAudit,
    pub inventory: Inventory,
    // NPC-only optional fields
    pub npc_memory: Option<NpcMemory>,
    pub npc_schedule: Option<NpcSchedule>,
    pub npc_goals: Option<NpcGoals>,
    pub disposition: Option<Disposition>,
    // Player-only optional fields
    pub is_player: bool,
    pub capabilities: Option<CapabilitySet>,
    pub transformation: Option<TransformationState>,
    pub knowledge: Option<KnowledgeInventory>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationSnapshotV1 {
    pub version: u32,
    pub clock: SimClock,
    pub world_map: WorldMap,
    pub settlements: SettlementDirectory,
    pub households: HouseholdDirectory,
    pub relationships: RelationshipLedger,
    pub reputation: ReputationRegistry,
    pub events: EventRing,
    pub next_citizen_id: u64,
    pub citizens: Vec<CitizenSnapshotV1>,
    pub seed: u64,
}

// ── V2 Authoritative Snapshot Types ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CitizenSnapshot {
    pub meta: CitizenMeta,
    pub demographics: Demographics,
    pub household_ref: HouseholdRef,
    pub settlement_ref: SettlementRef,
    pub occupation: OccupationProfile,
    pub finances: PersonalFinances,
    pub needs: PhysicalNeeds,
    pub mobility: MobilityProfile,
    pub kinship: Kinship,
    pub causal_audit: CausalAudit,
    pub inventory: Inventory,
    // NPC-only optional fields
    pub npc_memory: Option<NpcMemory>,
    pub npc_schedule: Option<NpcSchedule>,
    pub npc_goals: Option<NpcGoals>,
    pub disposition: Option<Disposition>,
    // Player-only optional fields
    pub is_player: bool,
    pub capabilities: Option<CapabilitySet>,
    pub transformation: Option<TransformationState>,
    pub knowledge: Option<KnowledgeInventory>,
    // VS2 Authoritative Components
    pub episodic_memory: Option<EpisodicMemory>,
    pub relational_ledger: Option<RelationalLedger>,
    pub epistemic_state: Option<EpistemicState>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationSnapshot {
    pub version: u32,
    pub clock: SimClock,
    pub world_map: WorldMap,
    pub settlements: SettlementDirectory,
    pub households: HouseholdDirectory,
    pub relationships: RelationshipLedger,
    pub reputation: ReputationRegistry,
    pub events: EventRing,
    pub next_citizen_id: u64,
    pub citizens: Vec<CitizenSnapshot>,
    pub seed: u64,
    // VS2 Authoritative World Resources
    pub pending_consequences: PendingConsequenceRegistry,
    pub return_digests: ReturnDigestLog,
    pub documents: DocumentRegistry,
    pub next_causal_id: u64,
}

/// Upward migration from Version 1 snapshot to Version 2 snapshot
pub fn migrate_v1_to_v2(v1: SimulationSnapshotV1) -> SimulationSnapshot {
    let mut citizens_v2 = Vec::with_capacity(v1.citizens.len());

    for c in v1.citizens {
        let (episodic, relational, epistemic) = if c.is_player {
            let mut ep = EpistemicState::new();
            if let Some(ref k) = c.knowledge {
                for &node in &k.nodes {
                    ep.known.insert(node as u16, (v1.clock.tick, 1));
                }
            }
            (
                None,
                Some(RelationalLedger::new()),
                Some(ep),
            )
        } else {
            let mut ledger = RelationalLedger::new();
            let base_rel = v1.relationships.get(c.meta.id, crate::types::CitizenId::PLAYER);
            if base_rel != 0 {
                ledger.adjust(crate::types::CitizenId::PLAYER, base_rel.clamp(-100, 100) as i8, 0, 0);
            }
            (
                Some(EpisodicMemory::new()),
                Some(ledger),
                Some(EpistemicState::new()),
            )
        };

        citizens_v2.push(CitizenSnapshot {
            meta: c.meta,
            demographics: c.demographics,
            household_ref: c.household_ref,
            settlement_ref: c.settlement_ref,
            occupation: c.occupation,
            finances: c.finances,
            needs: c.needs,
            mobility: c.mobility,
            kinship: c.kinship,
            causal_audit: c.causal_audit,
            inventory: c.inventory,
            npc_memory: c.npc_memory,
            npc_schedule: c.npc_schedule,
            npc_goals: c.npc_goals,
            disposition: c.disposition,
            is_player: c.is_player,
            capabilities: c.capabilities,
            transformation: c.transformation,
            knowledge: c.knowledge,
            episodic_memory: episodic,
            relational_ledger: relational,
            epistemic_state: epistemic,
        });
    }

    SimulationSnapshot {
        version: FORMAT_VERSION_V2,
        clock: v1.clock,
        world_map: v1.world_map,
        settlements: v1.settlements,
        households: v1.households,
        relationships: v1.relationships,
        reputation: v1.reputation,
        events: v1.events,
        next_citizen_id: v1.next_citizen_id,
        citizens: citizens_v2,
        seed: v1.seed,
        pending_consequences: PendingConsequenceRegistry::default(),
        return_digests: ReturnDigestLog::default(),
        documents: DocumentRegistry::default(),
        next_causal_id: 100,
    }
}

/// Serialize and write a simulation snapshot (Version 2) to a writer.
/// Format: MAGIC_V2(8) + VERSION(4) + DATA(n) + CRC32(4)
pub fn save_snapshot<W: Write>(
    snapshot: &SimulationSnapshot,
    writer: &mut W,
) -> io::Result<()> {
    let data = bincode::serialize(snapshot)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let mut hasher = CrcHasher::new();
    hasher.update(&data);
    let checksum = hasher.finalize();

    writer.write_all(MAGIC_V2)?;
    writer.write_all(&FORMAT_VERSION_V2.to_le_bytes())?;
    writer.write_all(&data)?;
    writer.write_all(&checksum.to_le_bytes())?;
    Ok(())
}

/// Serialize and write a simulation snapshot in legacy Version 1 format (for migration testing)
pub fn save_snapshot_v1<W: Write>(
    snapshot: &SimulationSnapshotV1,
    writer: &mut W,
) -> io::Result<()> {
    let data = bincode::serialize(snapshot)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let mut hasher = CrcHasher::new();
    hasher.update(&data);
    let checksum = hasher.finalize();

    writer.write_all(MAGIC_V1)?;
    writer.write_all(&FORMAT_VERSION_V1.to_le_bytes())?;
    writer.write_all(&data)?;
    writer.write_all(&checksum.to_le_bytes())?;
    Ok(())
}

/// Read and validate a simulation snapshot from a reader.
/// Supports both GODSEED2 (current) and GODSEED1 (with automatic upward migration).
pub fn load_snapshot<R: Read>(reader: &mut R) -> io::Result<SimulationSnapshot> {
    let mut header = [0u8; 12]; // 8 bytes magic + 4 bytes version
    reader.read_exact(&mut header).map_err(|e| {
        if e.kind() == io::ErrorKind::UnexpectedEof {
            io::Error::new(io::ErrorKind::InvalidData, "Save file truncated or too short")
        } else {
            e
        }
    })?;

    let magic = &header[0..8];
    let version = u32::from_le_bytes(header[8..12].try_into().unwrap());

    if magic != MAGIC_V2 && magic != MAGIC_V1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid save file magic: {:?}", String::from_utf8_lossy(magic)),
        ));
    }

    if (magic == MAGIC_V2 && version > FORMAT_VERSION_V2) || (magic == MAGIC_V1 && version > FORMAT_VERSION_V1) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unsupported future save format version {} (expected at most {})", version, FORMAT_VERSION_V2),
        ));
    }

    // Read remaining payload + checksum
    let mut all_bytes = Vec::new();
    reader.read_to_end(&mut all_bytes)?;

    if all_bytes.len() < 4 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Save file truncated or too short"));
    }

    let (data_bytes, crc_bytes) = all_bytes.split_at(all_bytes.len() - 4);
    let stored_crc = u32::from_le_bytes(crc_bytes.try_into().unwrap());

    let mut hasher = CrcHasher::new();
    hasher.update(data_bytes);
    let computed_crc = hasher.finalize();

    if stored_crc != computed_crc {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("CRC32 mismatch: stored={:#x} computed={:#x}", stored_crc, computed_crc),
        ));
    }

    if magic == MAGIC_V2 {
        if version == FORMAT_VERSION_V2 {
            let snapshot: SimulationSnapshot = bincode::deserialize(data_bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            Ok(snapshot)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid version {} for GODSEED2 format", version),
            ))
        }
    } else if magic == MAGIC_V1 {
        if version == FORMAT_VERSION_V1 {
            let snapshot_v1: SimulationSnapshotV1 = bincode::deserialize(data_bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            Ok(migrate_v1_to_v2(snapshot_v1))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unsupported version {} for GODSEED1 format", version),
            ))
        }
    } else {
        unreachable!()
    }
}

