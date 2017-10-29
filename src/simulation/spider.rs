use cgmath::{InnerSpace, MetricSpace, Vector2};
use rand::Rng;

use world::monster::spider::*;

const WANDERING_SPEED: f32 = 0.0006;
const TARGET_DISTANCE: f32 = 2.0;
const TARGET_CHANGE_INTERVAL: f32 = 1000.0;
const TARGET_CHANGE_CHANCE: u32 = 3;

pub fn simulate<R: Rng>(spiders: &mut SpiderSet, rng: &mut R, dt: f32) {
    simulate_target_changes(spiders, rng, dt);
    simulate_wanderings(spiders, dt);
}

fn simulate_target_changes<R: Rng>(spiders: &mut SpiderSet, rng: &mut R, dt: f32) {
    let targets = monster_field!(spiders, mut targets);
    let positions = monster_field!(spiders, positions);
    let since_target_changes = &mut spiders.state.since_target_changes;
    since_target_changes.catch_up(TARGET_CHANGE_INTERVAL, dt, || {
        for (target, position) in targets.iter_mut().zip(positions.iter()) {
            let chance = rng.gen_weighted_bool(TARGET_CHANGE_CHANCE);
            if chance {
                let target_relative = Vector2::new(
                    rng.gen_range(-1.0, 1.0),
                    rng.gen_range(-1.0, 1.0),
                ).normalize_to(TARGET_DISTANCE);
                *target = *position + target_relative;
            }
        }
    });
}

fn simulate_wanderings(spiders: &mut SpiderSet, dt: f32) {
    let targets = monster_field!(spiders, mut targets).iter_mut();
    let positions = monster_field!(spiders, mut positions).iter_mut();
    let angles = monster_field!(spiders, mut angles).iter_mut();
    for (target, (position, angle)) in targets.zip(positions.zip(angles)) {
        if position.distance(*target) > 0.1 {
            *position += WANDERING_SPEED * dt * (*target - *position).normalize();
            *angle = Vector2::new(1.0, 0.0).angle(*target - *position);
        }
    }
}
