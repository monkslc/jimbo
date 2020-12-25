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
        let (deactivated, activated, destroyed) = match orb.1 {
            LaserType::Red => (
                materials.orb_red_deactivated.clone(),
                materials.orb_red_activated.clone(),
                materials.orb_red_destroyed.clone(),
            ),
            LaserType::Blue => (
                materials.orb_blue_deactivated.clone(),
                materials.orb_blue_activated.clone(),
                materials.orb_blue_destroyed.clone(),
            ),
        };

        for laser in laser_q.iter() {
            let Laser(_, laser_type, end) = laser;
            if end == coord {
                if *laser_type == orb.1 && orb.0 != OrbState::Destroyed {
                    orb.0 = OrbState::Activated;
                    *material = activated;
                    continue 'outer;
                } else {
                    orb.0 = OrbState::Destroyed;
                    *material = destroyed;
                    continue 'outer;
                }
            }
        }

        if orb.0 != OrbState::Destroyed {
            *material = deactivated;
            orb.0 = OrbState::Deactivated;
        }
    }
}
