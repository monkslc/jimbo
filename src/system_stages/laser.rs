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
    level_size: Res<LevelSize>,
    opaque_q: Query<&Opaque>,
    refactor_q: Query<&Refactor>,
    laser_sources_q: Query<(&LaserSource, &Coordinate)>,
    mut lasers_q: Query<(&mut Laser, &Handle<Mesh>)>,
    coordinate_change_q: Query<(), Changed<Coordinate>>,
) {
    if coordinate_change_q.iter().next().is_none() {
        return;
    }

    let window = windows.get_primary().unwrap();
    for (mut laser, laser_mesh) in lasers_q.iter_mut() {
        if let Ok((LaserSource { direction, .. }, start)) = laser_sources_q.get(laser.source) {
            let (path, end) = compute_laser_path(
                window,
                &level_size,
                *start,
                *direction,
                &tracker,
                &opaque_q,
                &refactor_q,
            );
            let mesh = path_to_mesh(&path);
            let old_mesh = meshes.get_mut(laser_mesh).unwrap();
            *old_mesh = mesh;
            laser.end = end;
        }
    }
}

fn compute_laser_path(
    window: &Window,
    level_size: &Res<LevelSize>,
    start: Coordinate,
    direction: crate::Direction,
    tracker: &Res<EntityTracker>,
    opaque_q: &Query<&Opaque>,
    refactors_q: &Query<&Refactor>,
) -> (Path, Coordinate) {
    let mut direction = direction;
    let mut check_coordinate = start + direction.direction();
    let mut builder = Path::builder();
    let screen_space_start = coordinate_to_screen_space(start, window, level_size);
    builder.move_to(point(screen_space_start.x, screen_space_start.y));
    'outer: while check_coordinate.x >= 0
        && check_coordinate.x < (level_size.width as i32)
        && check_coordinate.y >= 0
        && check_coordinate.y < (level_size.height as i32)
    {
        if let Some(entities) = tracker.0.get(&check_coordinate) {
            for entity in entities {
                if opaque_q.get(*entity).is_ok() {
                    break 'outer;
                }

                if let Ok(Refactor { main_direction }) = refactors_q.get(*entity) {
                    let alt_direction = main_direction.rotated_90();

                    let new_direction = if main_direction.rotated_180() == direction {
                        alt_direction
                    } else if alt_direction.rotated_180() == direction {
                        *main_direction
                    } else {
                        break 'outer;
                    };
                    let screen_space =
                        coordinate_to_screen_space(check_coordinate, window, level_size);
                    builder.line_to(point(screen_space.x, screen_space.y));
                    direction = new_direction;
                }
            }
        }
        check_coordinate += direction.direction();
    }

    let screen_space = coordinate_to_screen_space(check_coordinate, window, level_size);
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
