use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path as FilePath;

use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct Tile;

pub struct MapBuilderPlugin;

impl Plugin for MapBuilderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("initial_level_load", SystemStage::parallel());
        app.add_startup_system_to_stage("initial_level_load", initial_level.system());
        app.add_system(change_level.system());
        app.add_system_to_stage(stage::PRE_UPDATE, level_change.system());
    }
}

pub fn change_level(
    keyboard_input: Res<Input<KeyCode>>,
    mut my_events: ResMut<Events<LevelChangeEvent>>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        println!("Changing to level 1");
        my_events.send(LevelChangeEvent(0));
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        println!("Changing to level 2");
        my_events.send(LevelChangeEvent(1));
    } else if keyboard_input.just_pressed(KeyCode::Key3) {
        println!("Changing to level 3");
        my_events.send(LevelChangeEvent(2));
    } else if keyboard_input.just_pressed(KeyCode::Key4) {
        println!("Changing to level 4");
        my_events.send(LevelChangeEvent(3));
    } else if keyboard_input.just_pressed(KeyCode::Key5) {
        println!("Changing to level 5");
        my_events.send(LevelChangeEvent(4));
    } else if keyboard_input.just_pressed(KeyCode::Key6) {
        println!("Changing to level 6");
        my_events.send(LevelChangeEvent(5));
    } else if keyboard_input.just_pressed(KeyCode::Key7) {
        println!("Changing to level 7");
        my_events.send(LevelChangeEvent(6));
    } else if keyboard_input.just_pressed(KeyCode::Key8) {
        println!("Changing to level 8");
        my_events.send(LevelChangeEvent(7));
    } else if keyboard_input.just_pressed(KeyCode::Key9) {
        println!("Changing to level 9");
        my_events.send(LevelChangeEvent(8));
    } else if keyboard_input.just_pressed(KeyCode::Key0) {
        println!("Changing to level 10");
        my_events.send(LevelChangeEvent(9));
    }
}

pub struct LevelChangeEvent(pub usize);

static LEVELS: [&str; 10] = [
    "levels/1.lvl",
    "levels/2.lvl",
    "levels/3.lvl",
    "levels/multi-color.lvl",
    "levels/playground-option-2.lvl",
    "levels/playground.lvl",
    "levels/scene-test.lvl",
    "levels/standoff.lvl",
    "levels/trapped-orb.lvl",
    "",
];

fn level_change(
    commands: &mut Commands,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut turn_counter: ResMut<TurnCounter>,
    mut undo_buffer: ResMut<UndoBuffer>,
    mut event_reader: Local<EventReader<LevelChangeEvent>>,
    events: Res<Events<LevelChangeEvent>>,
    entities: Query<Entity, With<LevelObject>>,
) {
    if let Some(latest_change) = event_reader.latest(&events) {
        turn_counter.0 = 0;
        undo_buffer.0.clear();
        for ent in entities.iter() {
            commands.despawn(ent);
        }

        load_level(
            std::path::Path::new(LEVELS[latest_change.0]),
            commands,
            &materials,
            &mut meshes,
        );
    }
}

fn initial_level(
    commands: &mut Commands,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    load_level(
        FilePath::new("levels/1.lvl"),
        commands,
        &materials,
        &mut meshes,
    );
}

fn load_level(
    path: &FilePath,
    mut commands: &mut Commands,
    materials: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let level_file =
        File::open(path).unwrap_or_else(|err| panic!("Failed to open level: {:?}\n{}", path, err));
    let reader = BufReader::new(level_file);

    let mut lines = reader.lines().enumerate();

    let (_, sizes) = lines.next().expect("expected level size");
    let sizes = sizes.expect("expected level sizes");
    let mut sizes = sizes
        .split('|')
        .map(|size| size.trim())
        .map(|size| size.parse::<i32>().expect("expected a number for the size"));

    let height = sizes.next().expect("expected level height");
    let _width = sizes.next().expect("expected level width");

    for (y, line) in lines {
        let y = y as i32;
        let line = line.unwrap_or_else(|err| panic!("Error loading level: {:?}\n{}", path, err));
        for (x, object) in line.split('|').enumerate() {
            let x = x as i32;
            let coord = Coordinate { x, y: height - y };
            spawn_tile(&mut commands, &materials, coord);
            let object = object.trim();

            match object {
                "W" => spawn_wall(commands, materials, coord),
                "C" => spawn_crate(commands, materials, coord),
                "P" => spawn_jimbo(commands, materials, coord),
                x if x.starts_with('R') => {
                    let mut chars = x.chars().skip(1);
                    let direction = match chars.next().expect("expected refactor direction") {
                        'U' => crate::Direction::Up,
                        'R' => crate::Direction::Right,
                        'D' => crate::Direction::Down,
                        'L' => crate::Direction::Left,
                        d => panic!("Unrecognized direction: {}", d),
                    };
                    spawn_refactor(commands, materials, direction, coord);
                }
                x if x.starts_with('O') => {
                    let mut chars = x.chars().skip(1);
                    let laser_type = match chars.next().expect("expected laser type") {
                        'R' => LaserType::Red,
                        'B' => LaserType::Blue,
                        t => panic!("Unrecognized laser type: {}", t),
                    };
                    spawn_orb(commands, materials, laser_type, coord);
                }
                x if x.starts_with('L') => {
                    let mut chars = x.chars().skip(1);
                    let laser_type = match chars.next().expect("expected laser type") {
                        'R' => LaserType::Red,
                        'B' => LaserType::Blue,
                        t => panic!("Unrecognized laser type: {}", t),
                    };

                    let laser_direction = match chars.next().expect("expected laser direction") {
                        'U' => crate::Direction::Up,
                        'R' => crate::Direction::Right,
                        'D' => crate::Direction::Down,
                        'L' => crate::Direction::Left,
                        d => panic!("Unrecognized laser direction: {:?}", d),
                    };

                    spawn_laser_source(
                        commands,
                        materials,
                        meshes,
                        laser_type,
                        laser_direction,
                        coord,
                    );
                }
                "_" => (),
                _ => panic!("Unrecognized level object: {}", object),
            }
        }
    }
}

pub fn spawn_tile(commands: &mut Commands, materials: &Res<Materials>, coordinate: Coordinate) {
    commands
        .spawn(SpriteBundle {
            material: materials.tile.clone(),
            ..Default::default()
        })
        .with(LevelObject)
        .with(Tile)
        .with(crate::Size {
            width: 0.1,
            height: 0.1,
        })
        .with(coordinate);
}

pub fn spawn_crate(commands: &mut Commands, materials: &Res<Materials>, coordinate: Coordinate) {
    commands
        .spawn(SpriteBundle {
            material: materials.crate_material.clone(),
            sprite: Sprite {
                size: Default::default(),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(LevelObject)
        .with(Crate)
        .with(Movable(true))
        .with(coordinate)
        .with(Opaque)
        .with(crate::Size {
            width: 0.7,
            height: 0.7,
        });
}

pub fn spawn_wall(commands: &mut Commands, materials: &Res<Materials>, coordinate: Coordinate) {
    commands
        .spawn(SpriteBundle {
            material: materials.wall.clone(),
            sprite: Sprite {
                size: Default::default(),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(LevelObject)
        .with(Wall)
        .with(Movable(false))
        .with(coordinate)
        .with(Opaque)
        .with(crate::Size {
            width: 1.0,
            height: 1.0,
        });
}

pub fn spawn_laser_source(
    commands: &mut Commands,
    materials: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>,
    laser_type: LaserType,
    direction: crate::Direction,
    coordinate: Coordinate,
) {
    let material = match (laser_type, direction) {
        (LaserType::Red, crate::Direction::Right) => materials.laser_source_red_right.clone(),
        (LaserType::Red, crate::Direction::Down) => materials.laser_source_red_down.clone(),
        (LaserType::Red, crate::Direction::Left) => materials.laser_source_red_left.clone(),
        (LaserType::Red, crate::Direction::Up) => materials.laser_source_red_up.clone(),
        (LaserType::Blue, crate::Direction::Right) => materials.laser_source_blue_right.clone(),
        (LaserType::Blue, crate::Direction::Down) => materials.laser_source_blue_down.clone(),
        (LaserType::Blue, crate::Direction::Left) => materials.laser_source_blue_left.clone(),
        (LaserType::Blue, crate::Direction::Up) => materials.laser_source_blue_up.clone(),
    };

    let source = commands
        .spawn(SpriteBundle {
            material,
            sprite: Sprite {
                size: Default::default(),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(LevelObject)
        .with(LaserSource(direction, laser_type))
        .with(Movable(true))
        .with(coordinate)
        .with(Opaque)
        .with(crate::Size {
            height: 1.0,
            width: 1.0,
        })
        .current_entity()
        .expect("should've had laser source");

    let material = match laser_type {
        LaserType::Red => materials.laser_red.clone(),
        LaserType::Blue => materials.laser_blue.clone(),
    };
    let mesh = meshes.add(Mesh::new(PrimitiveTopology::TriangleList));
    commands
        .spawn(SpriteBundle {
            material,
            mesh,
            sprite: Sprite {
                size: Vec2::new(1.0, 1.0),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(LevelObject)
        .with(Laser(source, laser_type, coordinate));
}

pub fn spawn_jimbo(commands: &mut Commands, materials: &Res<Materials>, coordinate: Coordinate) {
    commands
        .spawn(SpriteBundle {
            material: materials.jimbo.clone(),
            sprite: Sprite {
                size: Default::default(),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(LevelObject)
        .with(Jimbo)
        .with(Opaque)
        .with(coordinate)
        .with(crate::Size {
            width: 0.75,
            height: 0.75,
        });
}

pub fn spawn_orb(
    commands: &mut Commands,
    materials: &Res<Materials>,
    laser_type: LaserType,
    coordinate: Coordinate,
) {
    let material = match laser_type {
        LaserType::Red => materials.orb_red_deactivated.clone(),
        LaserType::Blue => materials.orb_blue_deactivated.clone(),
    };
    commands
        .spawn(SpriteBundle {
            material,
            sprite: Sprite {
                size: Default::default(),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(LevelObject)
        .with(Orb(false, laser_type))
        .with(Opaque)
        .with(Movable(false))
        .with(coordinate)
        .with(crate::Size {
            width: 0.4,
            height: 0.4,
        });
}

pub fn spawn_refactor(
    commands: &mut Commands,
    materials: &Res<Materials>,
    direction: crate::Direction,
    coordinate: Coordinate,
) {
    let material = match direction {
        crate::Direction::Right => materials.refactor_right.clone(),
        crate::Direction::Down => materials.refactor_down.clone(),
        crate::Direction::Left => materials.refactor_left.clone(),
        crate::Direction::Up => materials.refactor_up.clone(),
    };

    commands
        .spawn(SpriteBundle {
            material,
            sprite: Sprite {
                size: Default::default(),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(LevelObject)
        .with(Refactor)
        .with(Movable(true))
        .with(coordinate)
        .with(crate::Size {
            width: 0.9,
            height: 0.9,
        })
        .with(direction);
}
