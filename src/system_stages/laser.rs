use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;
use lyon::math::{point, Point};
use lyon::path::Path;
use lyon::tessellation::*;

use crate::system_stages::screen_transformations::coordinate_to_screen_space;
use crate::*;

pub type Geometry = VertexBuffers<[f32; 2], u16>;

pub const NAME: &str = "laser";

pub fn stage() -> SystemStage {
    let mut stage = SystemStage::parallel();
    stage.add_system(laser_path_adjustment.system());
    stage
}

fn laser_path_adjustment(
    mut meshes: ResMut<Assets<Mesh>>,
    windows: Res<Windows>,
    tracker: Res<EntityTracker>,
    opaque_q: Query<&Opaque>,
    coordinate_change_q: Query<(), Changed<Coordinate>>,
    refactor_q: Query<&crate::Direction, With<Refactor>>,
    laser_sources_q: Query<(&LaserSource, &Coordinate)>,
    mut lasers_q: Query<(&mut Laser, &Handle<Mesh>)>,
) {
    if coordinate_change_q.iter().next().is_none() {
        return;
    }

    let window = windows.get_primary().unwrap();
    for mut laser in lasers_q.iter_mut() {
        let Laser(source, _, _) = *(laser.0);
        let mesh_handle = laser.1;

        let (LaserSource(direction, _), start) = laser_sources_q
            .get(source)
            .expect("Laser should've had a source");
        let (path, end) =
            compute_laser_path(window, *start, *direction, &tracker, &opaque_q, &refactor_q);
        let mesh = path_to_mesh(&path);
        let old_mesh = meshes.get_mut(mesh_handle).unwrap();
        *old_mesh = mesh;
        laser.0 .2 = end;
    }
}

fn compute_laser_path(
    window: &Window,
    start: Coordinate,
    direction: crate::Direction,
    tracker: &Res<EntityTracker>,
    opaque_q: &Query<&Opaque>,
    refactors_q: &Query<&crate::Direction, With<Refactor>>,
) -> (Path, Coordinate) {
    let mut direction = direction;
    let mut check_coordinate = start + direction.direction();
    let mut builder = Path::builder();
    let screen_space_start = coordinate_to_screen_space(start, window);
    builder.move_to(point(screen_space_start.x, screen_space_start.y));
    // TODO: better map coord checks
    'outer: while check_coordinate.x < 10
        && check_coordinate.x > 0
        && check_coordinate.y > 0
        && check_coordinate.y < 10
    {
        if let Some(entities) = tracker.0.get(&check_coordinate) {
            for entity in entities {
                if opaque_q.get(*entity).is_ok() {
                    break 'outer;
                }

                if let Ok(refactor_direction) = refactors_q.get(*entity) {
                    let new_direction = match direction {
                        crate::Direction::Up => match refactor_direction {
                            crate::Direction::Down => crate::Direction::Left,
                            crate::Direction::Right => crate::Direction::Right,
                            _ => break 'outer,
                        },
                        crate::Direction::Right => match refactor_direction {
                            crate::Direction::Left => crate::Direction::Up,
                            crate::Direction::Down => crate::Direction::Down,
                            _ => break 'outer,
                        },
                        crate::Direction::Down => match refactor_direction {
                            crate::Direction::Up => crate::Direction::Right,
                            crate::Direction::Left => crate::Direction::Left,
                            _ => break 'outer,
                        },
                        crate::Direction::Left => match refactor_direction {
                            crate::Direction::Right => crate::Direction::Down,
                            crate::Direction::Up => crate::Direction::Up,
                            _ => break 'outer,
                        },
                    };
                    let screen_space = coordinate_to_screen_space(check_coordinate, window);
                    builder.line_to(point(screen_space.x, screen_space.y));
                    direction = new_direction;
                }
            }
        }
        check_coordinate += direction.direction();
    }
    let screen_space = coordinate_to_screen_space(check_coordinate, window);
    builder.line_to(point(screen_space.x, screen_space.y));
    (builder.build(), check_coordinate)
}

fn path_to_mesh(path: &Path) -> Mesh {
    let mut geometry = Geometry::new();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut tessellator = StrokeTessellator::new();
    tessellator
        .tessellate(
            path,
            &StrokeOptions::default().with_line_width(10.0),
            &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: StrokeAttributes| {
                pos.to_array()
            }),
        )
        .unwrap();

    let vertices_len = geometry.vertices.len();

    let normals = vec![[0.0, 0.0, 0.0]; vertices_len];
    let uv = vec![[0.0, 0.0]; vertices_len];

    mesh.set_indices(Some(Indices::U16(geometry.indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, geometry.vertices);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);

    mesh
}
