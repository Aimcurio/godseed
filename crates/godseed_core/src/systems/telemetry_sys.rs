/// Telemetry System — emit structured telemetry events each tick
use bevy_ecs::prelude::*;

use crate::components::{
    CitizenMeta, Demographics, PhysicalNeeds, PlayerMarker, TransformationState,
};
use crate::events::TelemetryEvent;
use crate::resources::TelemetryLog;
use crate::telemetry::TelemetryEventType;
use crate::types::SimClock;

pub fn telemetry_system(
    clock: Res<SimClock>,
    mut log: ResMut<TelemetryLog>,
    player_query: Query<
        (
            &CitizenMeta,
            &Demographics,
            &PhysicalNeeds,
            &TransformationState,
        ),
        With<PlayerMarker>,
    >,
) {
    // Only emit telemetry at meaningful intervals to avoid log bloat
    if clock.tick % 24 != 0 {
        return;
    } // Once per game-day

    let scenario_id = log.scenario_id;

    for (meta, demo, needs, transform) in player_query.iter() {
        if !meta.alive {
            continue;
        }

        log.emit(TelemetryEvent {
            tick: clock.tick,
            scenario_id,

            event_type: TelemetryEventType::PhysiologyEvent,
            actor: Some(0),
            target: None,
            location: None,
            action: None,
            outcome: Some(format!(
                "satiety={} health={} transform_stage={} progress={}",
                needs.satiety, demo.health, transform.stage, transform.progress
            )),
        });
    }
}
