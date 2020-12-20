use bevy::prelude::*;

use crate::*;

pub const NAME: &str = "movement";

pub fn stage() -> SystemStage {
    let mut system = SystemStage::parallel();
    system.add_system(jimbo_movement.system());
    system
}

pub fn jimbo_movement(
    keyboard_input: Res<Input<KeyCode>>,
    tracker: Res<EntityTracker>,
    mut q: QuerySet<(
        Query<(Entity, &Coordinate), With<Jimbo>>,
        Query<&mut Coordinate>,
        Query<&Movable>,
    )>,
) {
    let (jimbo, coordinate) = q.q0().iter().next().expect("Should always have jimbo");

    let direction = if keyboard_input.just_pressed(KeyCode::Left) {
        IVec2::new(-1, 0)
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        IVec2::new(1, 0)
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        IVec2::new(0, -1)
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        IVec2::new(0, 1)
    } else {
        return;
    };

    let mut check_coordinate = *coordinate + direction;
    let mut move_entities = vec![jimbo];
    'outer: while let Some(entities) = tracker.0.get(&check_coordinate) {
        let mut has_movable = false;
        for ent in entities {
            if let Ok(movable) = q.q2().get(*ent) {
                if !movable.0 {
                    move_entities = vec![];
                    break 'outer;
                }
                has_movable = true;
                move_entities.push(*ent);
            }
        }
        if !has_movable {
            break 'outer;
        }
        check_coordinate += direction;
    }

    for ent in move_entities.into_iter() {
        let mut coordinate = q
            .q1_mut()
            .get_mut(ent)
            .expect("This entity should have a coordinate");
        *coordinate += direction;
    }
}
