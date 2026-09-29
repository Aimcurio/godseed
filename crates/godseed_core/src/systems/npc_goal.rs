/// NPC Goal System — NPCs evaluate and pursue short-term goals

use bevy_ecs::prelude::*;

use crate::components::{CitizenMeta, NpcGoals, NpcSchedule, PersonalFinances, PhysicalNeeds};
use crate::components::PlayerMarker;
use crate::types::{NpcGoal, SimClock};

pub fn npc_goal_system(
    _clock: Res<SimClock>,
    mut query: Query<
        (&CitizenMeta, &mut NpcGoals, &PhysicalNeeds, &PersonalFinances, &NpcSchedule),
        Without<PlayerMarker>,
    >,
) {
    for (meta, mut goals, needs, finances, _schedule) in query.iter_mut() {
        if !meta.alive { continue; }

        // Decrement goal timer
        if goals.goal_ticks_remaining > 0 {
            goals.goal_ticks_remaining -= 1;
            continue;
        }

        // Evaluate new goal based on needs (priority order)
        let new_goal = if needs.satiety < 20 {
            Some(NpcGoal::SatisfyHunger)
        } else if needs.rest < 15 {
            Some(NpcGoal::Rest)
        } else if finances.coins < 5.0 {
            Some(NpcGoal::EarnMoney { target_amount: 20 })
        } else {
            // Follow routine
            Some(NpcGoal::CompleteWork)
        };

        if goals.active_goal != new_goal {
            goals.active_goal = new_goal;
            goals.goal_ticks_remaining = 24; // Evaluate every 24 ticks max
        }
    }
}
