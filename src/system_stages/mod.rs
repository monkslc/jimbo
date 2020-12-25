use bevy::prelude::*;

pub mod laser;

pub mod input;

pub mod tracking;

pub mod orb;

pub mod post_level_update;

pub mod screen_transformations;

pub struct SystemStagesPlugin;

impl Plugin for SystemStagesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(stage::UPDATE, input::NAME, input::stage());
        app.add_stage_after(input::NAME, "tracking-1", tracking::stage());

        app.add_stage_after("tracking-1", laser::NAME, laser::stage());
        app.add_stage_after(laser::NAME, "tracking-2", tracking::stage());

        app.add_stage_after("tracking-2", orb::NAME, orb::stage());

        app.add_stage_after(
            orb::NAME,
            post_level_update::NAME,
            post_level_update::stage(),
        );

        app.add_stage_after(
            post_level_update::NAME,
            screen_transformations::NAME,
            screen_transformations::stage(),
        );
    }
}
