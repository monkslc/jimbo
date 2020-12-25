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
        .add_plugins(DefaultPlugins)
        .add_plugin(events::EventPlugin)
        .add_plugin(resources::ResourcesPlugin)
        .add_plugin(StartupSystemPlugin)
        .add_plugin(SystemStagesPlugin)
        .run();
}
