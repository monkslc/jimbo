use bevy::prelude::*;
use bevy::render::pass::ClearColor;

use game::*;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_resource(WindowDescriptor {
            title: "Game!".to_string(),
            width: 1500.0,
            height: 700.0,
            ..Default::default()
        })
        .add_resource(TurnCounter::default())
        .add_resource(LevelSize::default())
        .add_resource(UndoBuffer::default())
        .add_event::<map::LevelChangeEvent>()
        .add_event::<system_stages::undo::UpdateLaserEvent>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(MaterialsPlugin)
        .add_plugin(MapBuilderPlugin)
        .add_stage_after(
            stage::UPDATE,
            system_stages::MOVEMENT,
            system_stages::movement::stage(),
        )
        .add_resource(EntityTracker::default())
        .add_stage_after(
            system_stages::MOVEMENT,
            system_stages::TRACKING,
            system_stages::tracking::stage(),
        )
        .add_stage_after(
            system_stages::TRACKING,
            system_stages::LASER,
            system_stages::laser::stage(),
        )
        // Right now we need to do tracking twice because the end coordinates of the laser changes
        // This is obviously not idea, but we will fix later
        .add_stage_after(
            system_stages::LASER,
            "tracking-2",
            system_stages::tracking::stage(),
        )
        .add_stage_after(
            "tracking-2",
            system_stages::ORB,
            system_stages::orb::stage(),
        )
        .add_stage_after(
            system_stages::ORB,
            system_stages::POST_LEVEL_UPDATE,
            system_stages::post_level_update::stage(),
        )
        .add_stage_after(
            system_stages::ORB,
            system_stages::SCREEN_TRANSFORMATIONS,
            system_stages::screen_transformations::stage(),
        )
        .add_stage_after(
            system_stages::POST_LEVEL_UPDATE,
            system_stages::UNDO,
            system_stages::undo::stage(),
        )
        .run();
}

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}
