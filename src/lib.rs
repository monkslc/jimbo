use bevy::prelude::*;
use euclid::Vector2D;

pub type IVec2 = Vector2D<i32, i32>;

pub mod components;
pub use components::*;
pub use components::{Direction, Size};

pub mod system_stages;

pub mod map;
pub use map::{MapBuilderPlugin, Tile};

#[derive(Debug, Clone, Default)]
pub struct Materials {
    crate_material: Handle<ColorMaterial>,
    jimbo: Handle<ColorMaterial>,
    tile: Handle<ColorMaterial>,
    laser_red: Handle<ColorMaterial>,
    laser_blue: Handle<ColorMaterial>,
    laser_source_blue_right: Handle<ColorMaterial>,
    laser_source_blue_down: Handle<ColorMaterial>,
    laser_source_blue_left: Handle<ColorMaterial>,
    laser_source_blue_up: Handle<ColorMaterial>,
    laser_source_red_right: Handle<ColorMaterial>,
    laser_source_red_down: Handle<ColorMaterial>,
    laser_source_red_left: Handle<ColorMaterial>,
    laser_source_red_up: Handle<ColorMaterial>,
    orb_red_activated: Handle<ColorMaterial>,
    orb_red_deactivated: Handle<ColorMaterial>,
    orb_blue_activated: Handle<ColorMaterial>,
    orb_blue_deactivated: Handle<ColorMaterial>,
    refactor_right: Handle<ColorMaterial>,
    refactor_down: Handle<ColorMaterial>,
    refactor_left: Handle<ColorMaterial>,
    refactor_up: Handle<ColorMaterial>,
    wall: Handle<ColorMaterial>,
}

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("materials", SystemStage::parallel());
        app.add_startup_system_to_stage("materials", create_materials.system());
    }
}

fn create_materials(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Materials {
        crate_material: materials.add(asset_server.load("crate.png").into()),
        jimbo: materials.add(asset_server.load("jimbo.png").into()),
        tile: materials.add(Color::rgb_u8(2, 95, 19).into()),
        laser_red: materials.add(Color::rgb_u8(255, 30, 30).into()),
        laser_blue: materials.add(Color::rgb_u8(30, 30, 255).into()),
        laser_source_blue_right: materials
            .add(asset_server.load("laser-source-blue-right.png").into()),
        laser_source_blue_down: materials
            .add(asset_server.load("laser-source-blue-down.png").into()),
        laser_source_blue_left: materials
            .add(asset_server.load("laser-source-blue-left.png").into()),
        laser_source_blue_up: materials.add(asset_server.load("laser-source-blue-up.png").into()),
        laser_source_red_right: materials
            .add(asset_server.load("laser-source-red-right.png").into()),
        laser_source_red_down: materials.add(asset_server.load("laser-source-red-down.png").into()),
        laser_source_red_left: materials.add(asset_server.load("laser-source-red-left.png").into()),
        laser_source_red_up: materials.add(asset_server.load("laser-source-red-up.png").into()),
        orb_red_activated: materials.add(asset_server.load("orb-red-activated.png").into()),
        orb_red_deactivated: materials.add(asset_server.load("orb-red-deactivated.png").into()),
        orb_blue_activated: materials.add(asset_server.load("orb-blue-activated.png").into()),
        orb_blue_deactivated: materials.add(asset_server.load("orb-blue-deactivated.png").into()),
        refactor_right: materials.add(asset_server.load("refactor-right.png").into()),
        refactor_down: materials.add(asset_server.load("refactor-down.png").into()),
        refactor_left: materials.add(asset_server.load("refactor-left.png").into()),
        refactor_up: materials.add(asset_server.load("refactor-up.png").into()),
        wall: materials.add(asset_server.load("wall.png").into()),
    });
}

#[derive(Debug, Copy, Clone, Default)]
pub struct TurnCounter(pub usize);

type UndoFn = Box<dyn Fn(&mut World) + Send + Sync + 'static>;

#[derive(Default)]
pub struct UndoBuffer(Vec<(usize, UndoFn)>);
