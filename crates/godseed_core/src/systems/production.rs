/// Production System — NPCs produce resources based on occupation + location

use bevy_ecs::prelude::*;

use crate::components::{CitizenMeta, NpcSchedule, OccupationProfile, SettlementRef};
use crate::components::PlayerMarker;
use crate::settlement::SettlementDirectory;
use crate::types::{NpcActivity, OccupationType, SimClock};

/// Per-tick production rates (per working NPC, per hour)
const FARMER_FOOD_PER_HOUR: f64 = 0.5;      // ~4/day
const FORESTER_TIMBER_PER_HOUR: f64 = 0.25; // ~2/day
const MINER_STONE_PER_HOUR: f64 = 0.15;
const ARTISAN_TOOLS_PER_HOUR: f64 = 0.08;
const HERBALIST_HERBS_PER_HOUR: f64 = 0.2;

pub fn production_system(
    clock: Res<SimClock>,
    mut settlements: ResMut<SettlementDirectory>,
    query: Query<(&CitizenMeta, &OccupationProfile, &NpcSchedule, &SettlementRef), Without<PlayerMarker>>,
) {
    let sid = SettlementDirectory::thornveil_id();

    for (meta, occ, schedule, settlement_ref) in query.iter() {
        if !meta.alive { continue; }
        if settlement_ref.settlement_id != sid { continue; }

        let hour = clock.hour();
        let (activity, _location) = schedule.activity_at_hour(hour);

        if activity != NpcActivity::Working { continue; }

        let (resource_ordinal, amount) = match occ.occupation {
            OccupationType::Farmer => (0u8, FARMER_FOOD_PER_HOUR * occ.productivity as f64),
            OccupationType::Forester => (1u8, FORESTER_TIMBER_PER_HOUR * occ.productivity as f64),
            OccupationType::Miner => (2u8, MINER_STONE_PER_HOUR * occ.productivity as f64),
            OccupationType::Artisan => (3u8, ARTISAN_TOOLS_PER_HOUR * occ.productivity as f64),
            OccupationType::Herbalist => (5u8, HERBALIST_HERBS_PER_HOUR * occ.productivity as f64),
            OccupationType::Laborer => {
                if _location == crate::types::LocationId(4) || _location == crate::types::LocationId(5) {
                    (0u8, 0.35 * occ.productivity as f64)
                } else if _location == crate::types::LocationId(11) {
                    (1u8, 0.2 * occ.productivity as f64)
                } else {
                    continue;
                }
            }
            _ => continue,
        };


        if let Some(settlement) = settlements.get_mut(sid) {
            settlement.add_stock(resource_ordinal, amount);
            if resource_ordinal == 0 {
                settlement.daily_food_production += amount as f32;
            }
        }
    }
}
