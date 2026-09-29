/// Labor System — weekly wage payments to NPCs

use bevy_ecs::prelude::*;

use crate::components::{CitizenMeta, OccupationProfile, PersonalFinances};
use crate::components::PlayerMarker;
use crate::settlement::SettlementDirectory;
use crate::types::OccupationType;

/// Base daily wages per occupation
fn base_daily_wage(occ: OccupationType) -> f32 {
    match occ {
        OccupationType::Farmer => 3.0,
        OccupationType::Forester => 3.5,
        OccupationType::Miner => 4.0,
        OccupationType::Artisan => 5.0,
        OccupationType::Merchant => 6.0,
        OccupationType::Innkeeper => 5.0,
        OccupationType::Herbalist => 4.5,
        OccupationType::Laborer => 2.5,
        OccupationType::Elder => 2.0,
        OccupationType::Unemployed => 0.0,
    }
}

/// Weekly: pay wages to working NPCs from settlement economy
pub fn labor_market_system(
    mut query: Query<(&CitizenMeta, &OccupationProfile, &mut PersonalFinances), Without<PlayerMarker>>,
    mut settlements: ResMut<SettlementDirectory>,
) {
    let sid = SettlementDirectory::thornveil_id();

    for (meta, occ, mut finances) in query.iter_mut() {
        if !meta.alive { continue; }

        let daily_wage = base_daily_wage(occ.occupation) * occ.skill_level as f32;
        let periodic_wage = daily_wage * (7.0 / 24.0);

        if periodic_wage > 0.0 {
            if let Some(_settlement) = settlements.get_mut(sid) {
                // Settlement pays from stockpile/coin pool (simplified)
                finances.coins += periodic_wage as f64;
                finances.last_income = periodic_wage;
            }
        }

    }
}
