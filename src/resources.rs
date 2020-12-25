use bevy::prelude::*;
use std::collections::HashMap;

use crate::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(EntityTracker::default());
        app.add_resource(LevelSize::default());
        app.add_resource(TurnCounter::default());
        app.add_resource(UndoBuffer::default());
    }
}

#[derive(Debug, Clone, Default)]
pub struct Materials {
    pub crate_material: Handle<ColorMaterial>,
    pub jimbo: Handle<ColorMaterial>,
    pub tile: Handle<ColorMaterial>,
    pub laser_red: Handle<ColorMaterial>,
    pub laser_blue: Handle<ColorMaterial>,
    pub laser_source_blue_right: Handle<ColorMaterial>,
    pub laser_source_blue_down: Handle<ColorMaterial>,
    pub laser_source_blue_left: Handle<ColorMaterial>,
    pub laser_source_blue_up: Handle<ColorMaterial>,
    pub laser_source_red_right: Handle<ColorMaterial>,
    pub laser_source_red_down: Handle<ColorMaterial>,
    pub laser_source_red_left: Handle<ColorMaterial>,
    pub laser_source_red_up: Handle<ColorMaterial>,
    pub orb_blue_activated: Handle<ColorMaterial>,
    pub orb_blue_deactivated: Handle<ColorMaterial>,
    pub orb_blue_destroyed: Handle<ColorMaterial>,
    pub orb_red_activated: Handle<ColorMaterial>,
    pub orb_red_deactivated: Handle<ColorMaterial>,
    pub orb_red_destroyed: Handle<ColorMaterial>,
    pub refactor_right: Handle<ColorMaterial>,
    pub refactor_down: Handle<ColorMaterial>,
    pub refactor_left: Handle<ColorMaterial>,
    pub refactor_up: Handle<ColorMaterial>,
    pub wall: Handle<ColorMaterial>,
}

#[derive(Debug, Clone, Default)]
pub struct EntityTracker(pub HashMap<Coordinate, Vec<Entity>>);

#[derive(Default)]
pub struct LevelSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct TurnCounter(pub usize);

pub type UndoFn = Box<dyn FnOnce(&mut World) + Send + Sync + 'static>;

#[derive(Default)]
pub struct UndoBuffer(pub Vec<(usize, UndoFn)>);
