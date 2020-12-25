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
    level_size: Res<LevelSize>,
    mut turn_counter: ResMut<TurnCounter>,
    mut undo_buffer: ResMut<UndoBuffer>,
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
    turn_counter.0 += 1;

    let mut check_coordinate = *coordinate + direction;
    let mut move_entities = vec![jimbo];
    'outer: while let Some(entities) = tracker.0.get(&check_coordinate) {
        let mut has_movable = false;
        for ent in entities {
            if let Ok(movable) = q.q2().get(*ent) {
                if !movable.0 {
                    move_entities.clear();
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

    if check_coordinate.x < 0
        || check_coordinate.x >= level_size.width as i32
        || check_coordinate.y < 0
        || check_coordinate.y >= level_size.height as i32
    {
        move_entities.clear();
    }

    for ent in move_entities.into_iter() {
        let undo_fn = Box::new(move |world: &mut World| {
            if let Ok(mut coordinate) = world.get_mut::<Coordinate>(ent) {
                *coordinate -= direction;
            }
        });
        undo_buffer.0.push((turn_counter.0, undo_fn));
        let mut coordinate = q
            .q1_mut()
            .get_mut(ent)
            .expect("This entity should have a coordinate");
        *coordinate += direction;
    }
}
