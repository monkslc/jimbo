use bevy::prelude::*;

use crate::{Coordinate, EntityTracker};

pub const NAME: &str = "tracking";

pub fn stage() -> SystemStage {
    let mut stage = SystemStage::parallel();
    stage.add_system(track_coordinates.system());
    stage
}

pub fn track_coordinates(
    mut tracker: ResMut<EntityTracker>,
    coordinate: Query<(Entity, &Coordinate)>,
) {
    tracker.0.clear();
    for (ent, coor) in coordinate.iter() {
        let entities = tracker.0.entry(*coor).or_insert(Vec::new());
        entities.push(ent);
    }
}
