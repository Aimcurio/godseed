/// Market System — weekly price discovery and household consumption

use bevy_ecs::prelude::*;

use crate::components::{CitizenMeta, PhysicalNeeds, SettlementRef};
use crate::components::PlayerMarker;
use crate::events::SimEvent;
use crate::resources::EventRing;
use crate::settlement::SettlementDirectory;
use crate::types::{ResourceType, SimClock};

/// Base prices for each resource (ResourceType ordinal)
const BASE_PRICES: [f32; 8] = [2.0, 5.0, 8.0, 12.0, 20.0, 6.0, 15.0, 10.0];
/// Estimated daily demand per NPC (for price calculation)
const DAILY_DEMAND: [f32; 8] = [0.5, 0.1, 0.05, 0.02, 0.01, 0.05, 0.01, 0.01];

/// Weekly: update market prices based on supply/demand
pub fn market_price_update_system(
    clock: Res<SimClock>,
    mut settlements: ResMut<SettlementDirectory>,
    mut event_ring: ResMut<EventRing>,
    query: Query<&CitizenMeta>,
) {
    let sid = SettlementDirectory::thornveil_id();
    let living_count = query.iter().filter(|m| m.alive).count() as f32;

    if let Some(settlement) = settlements.get_mut(sid) {
        for (ordinal, base) in BASE_PRICES.iter().enumerate() {
            let demand = DAILY_DEMAND[ordinal] * living_count * 7.0; // weekly demand
            let old_price = settlement.get_price(ordinal as u8);
            settlement.update_price(ordinal as u8, *base, demand / 7.0);
            let new_price = settlement.get_price(ordinal as u8);

            if (new_price - old_price).abs() > 0.5 {
                event_ring.emit(SimEvent::PriceChange {
                    resource: resource_from_ordinal(ordinal),
                    old_price,
                    new_price,
                    tick: clock.tick,
                });
            }
        }
        // Reset daily production tracker
        settlement.daily_food_production = 0.0;
    }
}

/// Weekly: NPCs consume food from the settlement stockpile
pub fn household_consumption_system(
    _clock: Res<SimClock>,
    mut settlements: ResMut<SettlementDirectory>,
    mut query: Query<(&CitizenMeta, &mut PhysicalNeeds, &SettlementRef), Without<PlayerMarker>>,
) {
    let sid = SettlementDirectory::thornveil_id();

    // Each NPC consumes 0.15 food units per 7-tick cycle (~0.5/day, 3.5/week)
    for (meta, mut needs, settlement_ref) in query.iter_mut() {
        if !meta.alive { continue; }
        if settlement_ref.settlement_id != sid { continue; }

        if let Some(settlement) = settlements.get_mut(sid) {
            if settlement.remove_stock(0, 0.15) {
                needs.satiety = (needs.satiety + 15).min(100);
            }
        }
    }
}



fn resource_from_ordinal(ordinal: usize) -> ResourceType {
    match ordinal {
        0 => ResourceType::Food,
        1 => ResourceType::Timber,
        2 => ResourceType::Stone,
        3 => ResourceType::Tools,
        4 => ResourceType::Luxury,
        5 => ResourceType::Herbs,
        6 => ResourceType::Ink,
        7 => ResourceType::Parchment,
        _ => ResourceType::Food,
    }
}
