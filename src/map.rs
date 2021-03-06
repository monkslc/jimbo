use bevy::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path as FilePath;

use crate::*;

pub fn load_level(
    path: &FilePath,
    mut commands: &mut Commands,
    materials: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>,
    level_size: &mut ResMut<LevelSize>,
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
        .map(|size| size.parse::<u32>().expect("expected a number for the size"));

    let height = sizes.next().expect("expected level height");
    let width = sizes.next().expect("expected level width");
    level_size.width = width;
    level_size.height = height;

    for (y, line) in lines {
        let y = y as i32;
        let line = line.unwrap_or_else(|err| panic!("Error loading level: {:?}\n{}", path, err));
        for (x, object) in line.split('|').enumerate() {
            let x = x as i32;
            let coord = Coordinate {
                x,
                y: ((height as i32) - y),
            };
            spawn_tile(&mut commands, &materials, coord);
            let object = object.trim();

            match object {
                "W" => spawn_wall(commands, materials, coord),
                "C" => spawn_crate(commands, materials, coord),
                "P" => spawn_jimbo(commands, materials, coord),
                "X" => {
                    let directions = vec![
                        crate::Direction::Up,
                        crate::Direction::Right,
                        crate::Direction::Down,
                        crate::Direction::Left,
                    ];
                    spawn_refactor(commands, materials, meshes, directions, coord);
                }
                x if x.starts_with('R') => {
                    let mut chars = x.chars().skip(1);
                    let direction = match chars.next().expect("expected splitter direction") {
                        'U' => crate::Direction::Up,
                        'R' => crate::Direction::Right,
                        'D' => crate::Direction::Down,
                        'L' => crate::Direction::Left,
                        d => panic!("Unrecognized direction: {}", d),
                    };
                    let directions = vec![direction, direction.rotated_90()];
                    spawn_refactor(commands, materials, meshes, directions, coord);
                }
                x if x.starts_with('S') => {
                    let mut chars = x.chars().skip(1);
                    let direction = match chars.next().expect("expected splitter direction") {
                        'U' => crate::Direction::Up,
                        'R' => crate::Direction::Right,
                        'D' => crate::Direction::Down,
                        'L' => crate::Direction::Left,
                        d => panic!("Unrecognized direction: {}", d),
                    };
                    let directions =
                        vec![direction, direction.rotated_90(), direction.rotated_180()];
                    spawn_refactor(commands, materials, meshes, directions, coord);
                }
                x if x.starts_with('O') => {
                    let mut chars = x.chars().skip(1);
                    let laser_type = match chars.next().expect("expected laser type") {
                        'R' => LaserType::Red,
                        'B' => LaserType::Blue,
                        'P' => LaserType::Purple,
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
        (LaserType::Purple, crate::Direction::Up) => materials.laser_source_purple_up.clone(),
        (LaserType::Purple, crate::Direction::Right) => materials.laser_source_purple_right.clone(),
        (LaserType::Purple, crate::Direction::Down) => materials.laser_source_purple_down.clone(),
        (LaserType::Purple, crate::Direction::Left) => materials.laser_source_purple_left.clone(),
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
        .with(LaserSource {
            direction,
            laser_type,
        })
        .with(Movable(true))
        .with(coordinate)
        .with(Opaque)
        .with(crate::Size {
            height: 1.0,
            width: 1.0,
        })
        .current_entity()
        .expect("should've had laser source");
    spawn_laser(commands, materials, meshes, laser_type, coordinate, source);
}

pub fn spawn_laser(
    commands: &mut Commands,
    materials: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>,
    laser_type: LaserType,
    end: Coordinate,
    source: Entity,
) -> Entity {
    let material = match laser_type {
        LaserType::Red => materials.laser_red.clone(),
        LaserType::Blue => materials.laser_blue.clone(),
        LaserType::Purple => materials.laser_purple.clone(),
    };
    let mesh = system_stages::laser::default_mesh();
    let mesh = meshes.add(mesh);
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
        .with(Laser {
            source,
            laser_type,
            end,
        })
        .current_entity()
        .unwrap()
}

pub fn spawn_jimbo(commands: &mut Commands, materials: &Res<Materials>, coordinate: Coordinate) {
    commands
        .spawn(SpriteBundle {
            material: materials.jimbo_down.clone(),
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
            width: 1.0,
            height: 1.0,
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
        LaserType::Purple => materials.orb_purple_deactivated.clone(),
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
        .with(Orb {
            state: OrbState::Deactivated,
            orb_type: laser_type,
        })
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
    meshes: &mut ResMut<Assets<Mesh>>,
    directions: Vec<crate::Direction>,
    coordinate: Coordinate,
) {
    let mut material_name =
        directions
            .iter()
            .fold(String::from("refactor"), |mut acc, direction| {
                acc.push('-');
                acc.push_str(direction.material_name());
                acc
            });
    material_name.push_str(".png");

    let material = materials.refactors.get(&material_name).unwrap();

    let source = commands
        .spawn(SpriteBundle {
            material: material.clone(),
            sprite: Sprite {
                size: Default::default(),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(LevelObject)
        .with(Movable(true))
        .with(coordinate)
        .with(crate::Size {
            width: 0.9,
            height: 0.9,
        })
        .current_entity()
        .unwrap();

    let refactor_directions = directions
        .into_iter()
        .map(|direction| {
            let outbound_laser = spawn_laser(
                commands,
                materials,
                meshes,
                LaserType::Red,
                coordinate,
                source,
            );
            RefactorDirection {
                direction,
                inbound_lasers: HashSet::new(),
                outbound_laser,
            }
        })
        .collect::<Vec<_>>();

    commands.insert_one(
        source,
        Refactor {
            directions: refactor_directions,
        },
    );
}
