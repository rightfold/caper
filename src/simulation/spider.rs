use cgmath::{InnerSpace, MetricSpace, Vector2, Zero};
use rand::Rng;

use world::monster::spider::*;

const WANDERING_SPEED: f32 = 0.6;

const VELOCITY_EPSILON: f32 = 1.5;
const VELOCITY_FRICTION: f32 = 5.0;

const TARGET_DISTANCE: f32 = 2.0;
const TARGET_CHANGE_INTERVAL: f32 = 1.0;
const TARGET_CHANGE_CHANCE: u32 = 3;

pub fn simulate<R: Rng>(spiders: &mut SpiderSet, rng: &mut R, dt: f32) {
    simulate_target_changes(spiders, rng, dt);
    simulate_velocities(spiders, dt);
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

fn simulate_velocities(spiders: &mut SpiderSet, dt: f32) {
    let velocities = monster_field!(spiders, mut velocities).iter_mut();
    let positions  = monster_field!(spiders, mut positions).iter_mut();
    for (velocity, position) in velocities.zip(positions) {
        *velocity *= f32::powf(1.0 / VELOCITY_FRICTION, dt);
        if velocity.magnitude() < VELOCITY_EPSILON {
            *velocity = Vector2::zero();
        }

        *position += dt * *velocity;
    }
}

fn simulate_wanderings(spiders: &mut SpiderSet, dt: f32) {
    let velocities = monster_field!(spiders, velocities).iter();
    let targets    = monster_field!(spiders, mut targets).iter_mut();
    let positions  = monster_field!(spiders, mut positions).iter_mut();
    let angles     = monster_field!(spiders, mut angles).iter_mut();
    for (velocity, (target, (position, angle))) in velocities.zip(targets.zip(positions.zip(angles))) {
        if !velocity.is_zero() {
            continue
        }
        if position.distance(*target) > 0.1 {
            *position += WANDERING_SPEED * dt * (*target - *position).normalize();
            *angle = Vector2::new(1.0, 0.0).angle(*target - *position);
        }
    }
}
