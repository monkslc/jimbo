use bevy::prelude::*;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<LevelChangeEvent>();
    }
}

pub struct LevelChangeEvent(pub usize);
