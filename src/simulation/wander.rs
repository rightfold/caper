use cgmath::{InnerSpace, MetricSpace, Rad, Vector2, Zero};
use rand::Rng;

use timing::Since;

const WANDERING_SPEED: f32 = 0.6;

const VELOCITY_EPSILON: f32 = 1.5;
const VELOCITY_FRICTION: f32 = 5.0;

const TARGET_DISTANCE: f32 = 2.0;
const TARGET_CHANGE_INTERVAL: f32 = 1.0;
const TARGET_CHANGE_CHANCE: u32 = 3;

pub fn simulate_wander<R: Rng>(rng: &mut R, dt: f32,
                               since_target_changes: &mut Since,
                               angles: &mut [Rad<f32>],
                               positions: &mut [Vector2<f32>],
                               targets: &mut [Vector2<f32>],
                               velocities: &mut [Vector2<f32>]) {
    simulate_target_changes(rng, dt, since_target_changes, positions, targets);
    simulate_velocities(dt, positions, velocities);
    simulate_wanderings(dt, angles, positions, targets, velocities);
}

fn simulate_target_changes<R: Rng>(rng: &mut R, dt: f32,
                                   since_target_changes: &mut Since,
                                   positions: &[Vector2<f32>],
                                   targets: &mut [Vector2<f32>]) {
    since_target_changes.catch_up(TARGET_CHANGE_INTERVAL, dt, || {
        let monsters = positions.iter().zip(targets.iter_mut());
        for (position, target) in monsters {
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

fn simulate_velocities(dt: f32, positions: &mut [Vector2<f32>],
                       velocities: &mut [Vector2<f32>]) {
    let monsters = positions.iter_mut().zip(velocities.iter_mut());
    for (position, velocity) in monsters {
        *velocity *= f32::powf(1.0 / VELOCITY_FRICTION, dt);
        if velocity.magnitude() < VELOCITY_EPSILON {
            *velocity = Vector2::zero();
        }

        *position += dt * *velocity;
    }
}

fn simulate_wanderings(dt: f32, angles: &mut [Rad<f32>],
                       positions: &mut [Vector2<f32>],
                       targets: &[Vector2<f32>],
                       velocities: &[Vector2<f32>]) {
    let monsters =
        angles.iter_mut()
        .zip(positions.iter_mut()
             .zip(targets.iter()
                  .zip(velocities.iter())));
    for (angle, (position, (target, velocity))) in monsters {
        if !velocity.is_zero() {
            continue
        }
        if position.distance(*target) > 0.1 {
            *position += WANDERING_SPEED * dt * (*target - *position).normalize();
            *angle = Vector2::new(1.0, 0.0).angle(*target - *position);
        }
    }
}
