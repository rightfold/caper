use rand::Rng;

use simulation::wander;
use world::monster::spider::*;

pub fn simulate<R: Rng>(spiders: &mut SpiderSet, rng: &mut R, dt: f32) {
    wander::simulate_wander(rng, dt, &mut spiders.scalars.since_target_changes,
                            soa_array!(spiders, mut angles),
                            soa_array!(spiders, mut positions),
                            soa_array!(spiders, mut targets),
                            soa_array!(spiders, mut velocities));
}
