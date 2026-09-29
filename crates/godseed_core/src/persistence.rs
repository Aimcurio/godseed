/// Godseed — Persistence (bincode + CRC32, inherited from CIVITAS-1M)

use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use crc32fast::Hasher as CrcHasher;

use crate::components::{
    CausalAudit, CapabilitySet, CitizenMeta, Demographics, Disposition, HouseholdRef,
    Inventory, Kinship, KnowledgeInventory, MobilityProfile, NpcGoals, NpcMemory, NpcSchedule,
    OccupationProfile, PersonalFinances, PhysicalNeeds, SettlementRef,
    TransformationState,
};
use crate::household::HouseholdDirectory;
use crate::resources::{EventRing, RelationshipLedger, ReputationRegistry};
use crate::settlement::SettlementDirectory;
use crate::types::SimClock;
use crate::world::WorldMap;


const MAGIC: &[u8; 8] = b"GODSEED1";
const FORMAT_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
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
}

#[derive(Serialize, Deserialize)]
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
}

/// Serialize and write a simulation snapshot to a writer.
/// Format: MAGIC(8) + VERSION(4) + DATA(n) + CRC32(4)
pub fn save_snapshot<W: Write>(
    snapshot: &SimulationSnapshot,
    writer: &mut W,
) -> io::Result<()> {
    let data = bincode::serialize(snapshot)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let mut hasher = CrcHasher::new();
    hasher.update(&data);
    let checksum = hasher.finalize();

    writer.write_all(MAGIC)?;
    writer.write_all(&FORMAT_VERSION.to_le_bytes())?;
    writer.write_all(&data)?;
    writer.write_all(&checksum.to_le_bytes())?;
    Ok(())
}

/// Read and validate a simulation snapshot from a reader.
pub fn load_snapshot<R: Read>(reader: &mut R) -> io::Result<SimulationSnapshot> {
    let mut magic = [0u8; 8];
    reader.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid save file magic"));
    }

    let mut version_bytes = [0u8; 4];
    reader.read_exact(&mut version_bytes)?;
    let version = u32::from_le_bytes(version_bytes);
    if version != FORMAT_VERSION {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Save file version {} not supported (expected {})", version, FORMAT_VERSION),
        ));
    }

    // Read all remaining data, last 4 bytes are checksum
    let mut all_bytes = Vec::new();

    reader.read_to_end(&mut all_bytes)?;

    if all_bytes.len() < 4 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Save file too short"));
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

    let snapshot: SimulationSnapshot = bincode::deserialize(data_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(snapshot)
}
