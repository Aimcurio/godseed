/// Relationship System — weekly passive decay

use bevy_ecs::prelude::*;
use crate::resources::RelationshipLedger;

/// Weekly: relationship values drift slightly toward 0 (forgetting).
/// Extreme values (-80..+80) decay more slowly.
pub fn relationship_decay_system(
    mut ledger: ResMut<RelationshipLedger>,
) {
    for value in ledger.values.values_mut() {
        if *value > 5 {
            *value -= 1;
        } else if *value < -5 {
            *value += 1;
        }
    }
}
