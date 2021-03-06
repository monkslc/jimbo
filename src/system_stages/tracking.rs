use bevy::prelude::*;

use crate::*;

pub fn stage() -> SystemStage {
    let mut stage = SystemStage::parallel();
    stage.add_system(track_coordinates.system());
    stage
}

pub fn track_coordinates(
    state: Res<AppState>,
    mut tracker: ResMut<EntityTracker>,
    coordinate: Query<(Entity, &Coordinate)>,
) {
    match *state {
        AppState::Level(_) => (),
        _ => return,
    }

    tracker.0.clear();
    for (ent, coor) in coordinate.iter() {
        let entities = tracker.0.entry(*coor).or_insert(Vec::new());
        entities.push(ent);
    }
}
