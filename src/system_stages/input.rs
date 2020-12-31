use bevy::prelude::*;

use crate::*;

pub const NAME: &str = "input";

pub fn stage() -> SystemStage {
    let mut system = SystemStage::parallel();
    system.add_system(jimbo_movement.system());
    system.add_system(undo.system());
    system.add_system(detect_level_change.system());
    system.add_system(detect_level_select.system());
    system.add_system(app_state_change_event.system());
    system
}

fn jimbo_movement(
    state: Res<AppState>,
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
    match *state {
        AppState::Level(_) => (),
        _ => return,
    }

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
        let undo = Box::new(move |world: &mut World| {
            if let Ok(mut coordinate) = world.get_mut::<Coordinate>(ent) {
                *coordinate -= direction;
            }
        });

        undo_buffer.0.push((turn_counter.0, undo));
        let mut coordinate = q
            .q1_mut()
            .get_mut(ent)
            .expect("This entity should have a coordinate");
        *coordinate += direction;
    }
}

fn undo(world: &mut World, resources: &mut Resources) {
    let state = resources
        .get::<AppState>()
        .expect("App should always have a state");

    match *state {
        AppState::Level(_) => (),
        _ => return,
    }

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

fn detect_level_select(
    state: Res<AppState>,
    mut my_events: ResMut<Events<AppStateChangeEvent>>,
    interaction_q: Query<(&Interaction, &AppStateChangeEvent), With<Button>>,
) {
    if *state != AppState::LevelSelect {
        return;
    }

    for (interaction, level_change) in interaction_q.iter() {
        match *interaction {
            Interaction::Clicked => {
                println!("Ok it was clicked though....");
                my_events.send(*level_change);
            }
            Interaction::Hovered => {
                // Do nothing... for now
            }
            Interaction::None => {
                // Do nothing... for now
            }
        }
    }
}

fn detect_level_change(
    keyboard_input: Res<Input<KeyCode>>,
    mut my_events: ResMut<Events<AppStateChangeEvent>>,
) {
    let level = if keyboard_input.just_pressed(KeyCode::Key1) {
        0
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        1
    } else if keyboard_input.just_pressed(KeyCode::Key3) {
        2
    } else if keyboard_input.just_pressed(KeyCode::Key4) {
        3
    } else if keyboard_input.just_pressed(KeyCode::Key5) {
        4
    } else if keyboard_input.just_pressed(KeyCode::Key6) {
        5
    } else if keyboard_input.just_pressed(KeyCode::Key7) {
        6
    } else if keyboard_input.just_pressed(KeyCode::Key8) {
        7
    } else if keyboard_input.just_pressed(KeyCode::Key9) {
        8
    } else if keyboard_input.just_pressed(KeyCode::Key0) {
        9
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        my_events.send(AppStateChangeEvent(AppState::LevelSelect));
        return;
    } else {
        return;
    };

    my_events.send(AppStateChangeEvent(AppState::Level(level)));
}

fn app_state_change_event(
    commands: &mut Commands,
    mut state: ResMut<AppState>,
    mut event_reader: Local<EventReader<AppStateChangeEvent>>,
    events: Res<Events<AppStateChangeEvent>>,
    ui_objects: Query<Entity, With<UiObject>>,
    level_objects: Query<Entity, With<LevelObject>>,
    materials: Res<Materials>,
    mut level_size: ResMut<LevelSize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut turn_counter: ResMut<TurnCounter>,
    mut undo_buffer: ResMut<UndoBuffer>,
    asset_server: Res<AssetServer>,
    color_materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(state_change) = event_reader.latest(&events) {
        match state_change.0 {
            AppState::Level(level_index) => {
                for ent in ui_objects.iter() {
                    commands.despawn_recursive(ent);
                }

                for ent in level_objects.iter() {
                    commands.despawn_recursive(ent);
                }

                turn_counter.0 = 0;
                undo_buffer.0.clear();

                map::load_level(
                    std::path::Path::new(LEVELS[level_index]),
                    commands,
                    &materials,
                    &mut meshes,
                    &mut level_size,
                );
            }
            AppState::LevelSelect => {
                for ent in level_objects.iter() {
                    commands.despawn_recursive(ent);
                }

                startup_systems::load_level_selector(commands, asset_server, color_materials);
            }
        }
        *state = state_change.0;
    }
}
