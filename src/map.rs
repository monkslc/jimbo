use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;
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
        .with(Laser {
            source,
            laser_type,
            end: coordinate,
        });
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
        .with(Refactor {
            main_direction: direction,
        })
        .with(Movable(true))
        .with(coordinate)
        .with(crate::Size {
            width: 0.9,
            height: 0.9,
        });
}
