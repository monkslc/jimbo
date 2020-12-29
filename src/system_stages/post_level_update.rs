use bevy::prelude::*;

use crate::*;

pub const NAME: &str = "post-level-update";

pub fn stage() -> SystemStage {
    let mut stage = SystemStage::parallel();
    stage.add_system(level_completed.system());
    stage
}

pub fn level_completed(laser_changed: Query<(), Changed<Laser>>, orbs: Query<&Orb>) {
    if laser_changed.iter().next().is_none() {
        return;
    }

    for orb in orbs.iter() {
        if orb.state != OrbState::Activated {
            return;
        }
    }

    println!("Level Complete");
}
