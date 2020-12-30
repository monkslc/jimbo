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
    materials: Res<Materials>,
    opaque_q: Query<&Opaque>,
    mut refactor_q: Query<(Entity, &mut Refactor, &Coordinate)>,
    laser_sources_q: Query<(&LaserSource, &Coordinate)>,
    mut lasers_q: Query<(Entity, &mut Laser, &Handle<Mesh>)>,
    coordinate_change_q: Query<(), Changed<Coordinate>>,
    mut laser_material_q: Query<&mut Handle<ColorMaterial>>,
) {
    if coordinate_change_q.iter().next().is_none() {
        return;
    }

    for (_, mut refactor, start) in refactor_q.iter_mut() {
        refactor.inbound_main.clear();
        refactor.inbound_alt.clear();

        let (_, mut laser, mesh_handle) = lasers_q.get_mut(refactor.outbound_main).unwrap();
        let old_mesh = meshes.get_mut(mesh_handle).unwrap();
        let new_mesh = default_mesh();
        *old_mesh = new_mesh;
        laser.end = *start;

        let (_, mut laser, mesh_handle) = lasers_q.get_mut(refactor.outbound_alt).unwrap();
        let old_mesh = meshes.get_mut(mesh_handle).unwrap();
        let new_mesh = default_mesh();
        *old_mesh = new_mesh;
        laser.end = *start;
    }

    let window = windows.get_primary().unwrap();
    let mut laser_direction_changes = Vec::new();
    for (laser_id, mut laser, laser_mesh) in lasers_q.iter_mut() {
        if let Ok((LaserSource { direction, .. }, start)) = laser_sources_q.get(laser.source) {
            update_laser(
                &mut meshes,
                laser_id,
                &mut laser,
                laser_mesh,
                window,
                &level_size,
                &materials,
                *start,
                *direction,
                &tracker,
                &opaque_q,
                &mut refactor_q,
                &mut laser_direction_changes,
                &mut laser_material_q,
            );
        }
    }

    while let Some(refactor_id) = laser_direction_changes.pop() {
        if let Ok((_, refactor, coordinate)) = refactor_q.get_mut(refactor_id) {
            if !refactor.inbound_main.is_empty() && !refactor.inbound_alt.is_empty() {
                continue;
            }

            if !refactor.inbound_main.is_empty() {
                let laser_type = LaserType::amalgamate(&refactor.inbound_main);
                let (_, mut laser, laser_mesh) = lasers_q.get_mut(refactor.outbound_alt).unwrap();
                laser.laser_type = laser_type;
                update_laser(
                    &mut meshes,
                    refactor.outbound_alt,
                    &mut laser,
                    laser_mesh,
                    window,
                    &level_size,
                    &materials,
                    *coordinate,
                    refactor.outbound_alt(),
                    &tracker,
                    &opaque_q,
                    &mut refactor_q,
                    &mut laser_direction_changes,
                    &mut laser_material_q,
                );
            } else if !refactor.inbound_alt.is_empty() {
                let laser_type = LaserType::amalgamate(&refactor.inbound_alt);
                let (_, mut laser, laser_mesh) = lasers_q.get_mut(refactor.outbound_main).unwrap();
                laser.laser_type = laser_type;
                update_laser(
                    &mut meshes,
                    refactor.outbound_main,
                    &mut laser,
                    laser_mesh,
                    window,
                    &level_size,
                    &materials,
                    *coordinate,
                    refactor.outbound_main(),
                    &tracker,
                    &opaque_q,
                    &mut refactor_q,
                    &mut laser_direction_changes,
                    &mut laser_material_q,
                );
            }
        }
    }
}

fn update_laser(
    meshes: &mut ResMut<Assets<Mesh>>,
    laser_id: Entity,
    laser: &mut Laser,
    laser_mesh: &Handle<Mesh>,
    window: &Window,
    level_size: &Res<LevelSize>,
    materials: &Res<Materials>,
    start: Coordinate,
    direction: crate::Direction,
    tracker: &Res<EntityTracker>,
    opaque_q: &Query<&Opaque>,
    refactors_q: &mut Query<(Entity, &mut Refactor, &Coordinate)>,
    refactor_stack: &mut Vec<Entity>,
    laser_material_q: &mut Query<&mut Handle<ColorMaterial>>,
) {
    let (path, end) = compute_laser_path(
        laser.laser_type,
        window,
        level_size,
        start,
        direction,
        tracker,
        opaque_q,
        refactors_q,
        refactor_stack,
    );

    let mesh = path_to_mesh(&path);
    let old_mesh = meshes.get_mut(laser_mesh).unwrap();
    *old_mesh = mesh;
    laser.end = end;

    let material = match laser.laser_type {
        LaserType::Red => materials.laser_red.clone(),
        LaserType::Blue => materials.laser_blue.clone(),
    };

    let mut old_material = laser_material_q.get_mut(laser_id).unwrap();
    *old_material = material;
}

fn compute_laser_path(
    laser_type: LaserType,
    window: &Window,
    level_size: &Res<LevelSize>,
    start: Coordinate,
    direction: crate::Direction,
    tracker: &Res<EntityTracker>,
    opaque_q: &Query<&Opaque>,
    refactors_q: &mut Query<(Entity, &mut Refactor, &Coordinate)>,
    refactor_stack: &mut Vec<Entity>,
) -> (Path, Coordinate) {
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

                if let Ok((refactor_id, mut refactor, _)) = refactors_q.get_mut(*entity) {
                    let main_direction = refactor.inbound_main();
                    let alt_direction = refactor.inbound_alt();
                    if direction == main_direction {
                        refactor.inbound_main.insert(laser_type);
                    } else if direction == alt_direction {
                        refactor.inbound_alt.insert(laser_type);
                    }
                    refactor_stack.push(refactor_id);
                    break 'outer;
                }
            }
        }
        check_coordinate += direction.direction();
    }

    let screen_space = coordinate_to_screen_space(check_coordinate, window, level_size);
    builder.line_to(point(screen_space.x, screen_space.y));
    (builder.build(), check_coordinate)
}

pub fn path_to_mesh(path: &Path) -> Mesh {
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

pub fn default_mesh() -> Mesh {
    let path = Path::builder().build();
    crate::system_stages::laser::path_to_mesh(&path)
}
