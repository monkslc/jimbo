use bevy::prelude::*;

use crate::*;

pub const NAME: &str = "orb";

pub fn stage() -> SystemStage {
    let mut stage = SystemStage::parallel();
    stage.add_system(orb_update.system());
    stage
}

pub fn orb_update(
    materials: Res<Materials>,
    laser_changed: Query<(), Changed<Laser>>,
    laser_q: Query<&Laser>,
    mut orb_q: Query<(&mut Orb, &Coordinate, &mut Handle<ColorMaterial>)>,
) {
    if laser_changed.iter().next().is_none() {
        return;
    }

    'outer: for (mut orb, coord, mut material) in orb_q.iter_mut() {
        let (deactivated, activated) = match orb.1 {
            LaserType::Red => (
                materials.orb_red_deactivated.clone(),
                materials.orb_red_activated.clone(),
            ),
            LaserType::Blue => (
                materials.orb_blue_deactivated.clone(),
                materials.orb_blue_activated.clone(),
            ),
        };
        for laser in laser_q.iter() {
            let Laser(_, laser_type, end) = laser;
            if end == coord && *laser_type == orb.1 {
                orb.0 = true;
                *material = activated;
                continue 'outer;
            }
        }

        *material = deactivated;
        orb.0 = false;
    }
}
