use bevy::prelude::*;
use std::collections::HashMap;
use std::path::Path as FilePath;

use crate::*;

pub struct StartupSystemPlugin;

impl Plugin for StartupSystemPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(initial_setup.system());

        app.add_startup_stage("materials", SystemStage::parallel());
        app.add_startup_system_to_stage("materials", create_materials.system());

        app.add_startup_stage("initial_level", SystemStage::serial());
        app.add_startup_system_to_stage("initial_level", initial_level.system());
    }
}

fn initial_setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn initial_level(
    commands: &mut Commands,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut level_size: ResMut<LevelSize>,
) {
    map::load_level(
        FilePath::new("levels/1.lvl"),
        commands,
        &materials,
        &mut meshes,
        &mut level_size,
    );
}

fn create_materials(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut mats = Materials {
        crate_material: materials.add(asset_server.load("crate.png").into()),
        jimbo: materials.add(asset_server.load("jimbo.png").into()),
        tile: materials.add(Color::rgb_u8(2, 95, 19).into()),
        laser_blue: materials.add(Color::rgb_u8(30, 30, 255).into()),
        laser_purple: materials.add(Color::rgb_u8(150, 30, 255).into()),
        laser_red: materials.add(Color::rgb_u8(255, 30, 30).into()),
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
        laser_source_purple_right: materials
            .add(asset_server.load("laser-source-purple-right.png").into()),
        laser_source_purple_down: materials
            .add(asset_server.load("laser-source-purple-down.png").into()),
        laser_source_purple_left: materials
            .add(asset_server.load("laser-source-purple-left.png").into()),
        laser_source_purple_up: materials
            .add(asset_server.load("laser-source-purple-up.png").into()),
        orb_blue_activated: materials.add(asset_server.load("orb-blue-activated.png").into()),
        orb_blue_deactivated: materials.add(asset_server.load("orb-blue-deactivated.png").into()),
        orb_blue_destroyed: materials.add(asset_server.load("orb-blue-destroyed.png").into()),
        orb_red_activated: materials.add(asset_server.load("orb-red-activated.png").into()),
        orb_red_deactivated: materials.add(asset_server.load("orb-red-deactivated.png").into()),
        orb_red_destroyed: materials.add(asset_server.load("orb-red-destroyed.png").into()),
        orb_purple_activated: materials.add(asset_server.load("orb-purple-activated.png").into()),
        orb_purple_deactivated: materials
            .add(asset_server.load("orb-purple-deactivated.png").into()),
        orb_purple_destroyed: materials.add(asset_server.load("orb-purple-destroyed.png").into()),
        refactor_right: materials.add(asset_server.load("refactor-right.png").into()),
        refactor_down: materials.add(asset_server.load("refactor-down.png").into()),
        refactor_left: materials.add(asset_server.load("refactor-left.png").into()),
        refactor_up: materials.add(asset_server.load("refactor-up.png").into()),
        refactors: HashMap::new(),
        wall: materials.add(asset_server.load("wall.png").into()),
    };

    for entry in std::fs::read_dir("assets").unwrap() {
        let entry = entry.unwrap();
        let asset = entry.file_name();
        let material_name = asset.to_str().unwrap();
        if material_name.starts_with("refactor") {
            let material = materials.add(asset_server.load(material_name).into());
            mats.refactors.insert(material_name.to_string(), material);
        }
    }

    commands.insert_resource(mats);
}
