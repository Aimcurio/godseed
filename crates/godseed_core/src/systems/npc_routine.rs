/// NPC Routine System — move NPCs through their daily schedules
use bevy_ecs::prelude::*;

use crate::components::PlayerMarker;
use crate::components::{CitizenMeta, NpcSchedule, SettlementRef};
use crate::types::SimClock;

pub fn npc_routine_system(
    clock: Res<SimClock>,
    mut query: Query<(&CitizenMeta, &mut NpcSchedule, &mut SettlementRef), Without<PlayerMarker>>,
) {
    let hour = clock.hour();

    for (meta, mut schedule, mut settlement_ref) in query.iter_mut() {
        if !meta.alive {
            continue;
        }

        let (new_activity, new_location) = schedule.activity_at_hour(hour);

        // Update if changed
        if schedule.current_activity != new_activity
            || settlement_ref.current_location != new_location
        {
            schedule.current_activity = new_activity;
            settlement_ref.current_location = new_location;
        }
    }
}
