use bevy::prelude::*;

use crate::*;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AppStateChangeEvent>();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AppStateChangeEvent(pub AppState);
