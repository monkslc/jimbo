use bevy::prelude::*;

use crate::*;

pub const NAME: &str = "undo";

pub fn stage() -> SystemStage {
    let mut system = SystemStage::serial();
    system.add_system(undo.system());
    // TODO: this is the third time we call track coordinates
    // Its needed here because we need to get the updated coordinates
    // from the undo before doing the laser events
    // We will improve this in the future
    system.add_system(system_stages::tracking::track_coordinates.system());
    system.add_system(update_laser_event.system());
    system
}

pub struct UpdateLaserEvent;

fn undo(world: &mut World, resources: &mut Resources) {
    let input = resources
        .get::<Input<KeyCode>>()
        .expect("Input resource should have been available");
    if !input.just_pressed(KeyCode::Z) {
        return;
    }

    let mut current_turn = resources
        .get_mut::<TurnCounter>()
        .expect("Should've had the current turn");

    if current_turn.0 == 0 {
        return;
    }

    let mut events = resources
        .get_mut::<Events<UpdateLaserEvent>>()
        .expect("Update Laser Event should've been a resource");
    events.send(UpdateLaserEvent);

    let mut undo_buffer = resources
        .get_mut::<UndoBuffer>()
        .expect("UndoBuffer should've been available");

    while let Some(undo) = undo_buffer.0.last() {
        if undo.0 == current_turn.0 {
            let func = undo_buffer.0.pop().unwrap().1;
            func(world);
        } else {
            break;
        }
    }

    current_turn.0 -= 1;
}

pub fn update_laser_event(
    meshes: ResMut<Assets<Mesh>>,
    windows: Res<Windows>,
    tracker: Res<EntityTracker>,
    level_size: Res<LevelSize>,
    opaque_q: Query<&Opaque>,
    refactor_q: Query<&crate::Direction, With<Refactor>>,
    laser_sources_q: Query<(&LaserSource, &Coordinate)>,
    lasers_q: Query<(&mut Laser, &Handle<Mesh>)>,
    events: Res<Events<UpdateLaserEvent>>,
    mut event_reader: Local<EventReader<UpdateLaserEvent>>,
) {
    if event_reader.latest(&events).is_some() {
        system_stages::laser::update_lasers(
            meshes,
            windows,
            tracker,
            level_size,
            opaque_q,
            refactor_q,
            laser_sources_q,
            lasers_q,
        );
    }
}
