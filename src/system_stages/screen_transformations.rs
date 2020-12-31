use bevy::prelude::*;

use crate::*;

pub const NAME: &str = "screen-transformations";

pub fn stage() -> SystemStage {
    let mut stage = SystemStage::parallel();
    stage.add_system(size_scaling.system());
    stage.add_system(position_translation.system());
    stage
}

fn size_scaling(
    state: Res<AppState>,
    windows: Res<Windows>,
    level_size: Res<LevelSize>,
    mut q: Query<(&crate::Size, &mut Sprite)>,
) {
    match *state {
        AppState::Level(_) => (),
        _ => return,
    }

    let window = windows.get_primary().unwrap();
    let tile_size = get_tile_size(window, &level_size);
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width * tile_size.x,
            sprite_size.height * tile_size.y,
        );
    }
}

fn position_translation(
    state: Res<AppState>,
    windows: Res<Windows>,
    level_size: Res<LevelSize>,
    mut q: Query<(&Coordinate, &mut Transform)>,
) {
    match *state {
        AppState::Level(_) => (),
        _ => return,
    }

    let window = windows.get_primary().unwrap();
    let tile_size = get_tile_size(window, &level_size);
    let bottom_left = Vec2::new(window.width() as f32 / -2.0, window.height() as f32 / -2.0);
    let center_sprite_adjustment = tile_size / 2.0;

    for (coordinate, mut transform) in q.iter_mut() {
        let pos = bottom_left + coordinate.scale(tile_size) + center_sprite_adjustment;
        transform.translation = pos.extend(0.0);
    }
}

pub fn get_tile_size(window: &Window, level_size: &Res<LevelSize>) -> Vec2 {
    let tile_width = window.width() as f32 / (level_size.width as f32);
    let tile_height = window.height() as f32 / (level_size.height as f32);
    let min = tile_height.min(tile_width);
    Vec2::new(min, min)
}

pub fn coordinate_to_screen_space(
    coord: Coordinate,
    window: &Window,
    level_size: &Res<LevelSize>,
) -> Vec2 {
    let tile_size = get_tile_size(window, level_size);
    let bottom_left = Vec2::new(window.width() as f32 / -2.0, window.height() / -2.0 as f32);
    let center_adjustment = tile_size / 2.0;
    bottom_left + coord.scale(tile_size) + center_adjustment
}
