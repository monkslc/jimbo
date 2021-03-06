use bevy::prelude::*;

use crate::*;

pub const NAME: &str = "orb";

pub fn stage() -> SystemStage {
    let mut stage = SystemStage::parallel();
    stage.add_system(orb_update.system());
    stage
}

pub fn orb_update(
    state: Res<AppState>,
    keyboard_input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    turn_counter: Res<TurnCounter>,
    mut undo_buffer: ResMut<UndoBuffer>,
    laser_changed: Query<(), Changed<Laser>>,
    laser_q: Query<&Laser>,
    mut orb_q: Query<(Entity, &mut Orb, &Coordinate, &mut Handle<ColorMaterial>)>,
) {
    match *state {
        AppState::Level(_) => (),
        _ => return,
    }

    if laser_changed.iter().next().is_none() {
        return;
    }

    let should_push_undo_buffer = !keyboard_input.just_pressed(KeyCode::Z);

    'outer: for (entity, mut orb, coord, mut material) in orb_q.iter_mut() {
        let original_material = material.clone();
        let original_state = orb.state;

        let undo_fn = Box::new(move |world: &mut World| {
            if let Ok(mut orb) = world.get_mut::<Orb>(entity) {
                orb.state = original_state;
            }

            if let Ok(mut material) = world.get_mut::<Handle<ColorMaterial>>(entity) {
                *material = original_material;
            }
        });

        let (deactivated, activated, destroyed) = match orb.orb_type {
            LaserType::Red => (
                materials.orb_red_deactivated.clone(),
                materials.orb_red_activated.clone(),
                materials.orb_red_destroyed.clone(),
            ),
            LaserType::Blue => (
                materials.orb_blue_deactivated.clone(),
                materials.orb_blue_activated.clone(),
                materials.orb_blue_destroyed.clone(),
            ),
            LaserType::Purple => (
                materials.orb_purple_deactivated.clone(),
                materials.orb_purple_activated.clone(),
                materials.orb_purple_destroyed.clone(),
            ),
        };

        for laser in laser_q.iter() {
            if laser.end == *coord {
                if laser.laser_type == orb.orb_type && orb.state != OrbState::Destroyed {
                    orb.state = OrbState::Activated;
                    *material = activated;
                } else {
                    orb.state = OrbState::Destroyed;
                    *material = destroyed;
                }

                if should_push_undo_buffer {
                    undo_buffer.0.push((turn_counter.0, undo_fn));
                }

                continue 'outer;
            }
        }

        if orb.state != OrbState::Destroyed {
            *material = deactivated;
            orb.state = OrbState::Deactivated;

            if should_push_undo_buffer {
                undo_buffer.0.push((turn_counter.0, undo_fn));
            }
        }
    }
}
