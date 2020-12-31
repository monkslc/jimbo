use euclid::Vector2D;

pub type IVec2 = Vector2D<i32, i32>;

pub mod components;
pub use components::*;
pub use components::{Direction, Size};

pub mod events;
pub use events::*;

pub mod map;

pub mod resources;
pub use resources::*;

pub mod startup_systems;
pub use startup_systems::StartupSystemPlugin;

pub mod system_stages;
pub use system_stages::SystemStagesPlugin;

pub static LEVELS: [&str; 10] = [
    "levels/1.lvl",
    "levels/2.lvl",
    "levels/3.lvl",
    "levels/multi-color.lvl",
    "levels/playground-option-2.lvl",
    "levels/playground.lvl",
    "levels/scene-test.lvl",
    "levels/standoff.lvl",
    "levels/trapped-orb.lvl",
    "levels/nand.lvl",
];
