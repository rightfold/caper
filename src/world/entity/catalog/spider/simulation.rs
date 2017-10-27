use cgmath::{Rad, Vector2};
use rand::Rng;

use world::entity::catalog::spider::*;

pub const STATE_CHANGE_CHANCE: u32 = 500;

pub const ROTATION_SPEED: Deg<f32> = Deg(0.1);

pub fn simulate_one<R: Rng>(rng: &mut R, dt: f32, position: &mut Vector2<f32>,
                            angle: &mut Rad<f32>, state: &mut SpiderState) {
    *position = simulate_position(dt, *position, *state);
    *angle = simulate_angle(dt, *angle, state.rotation_state());
    *state = simulate_state(rng, dt, *state);
}

fn simulate_position(_dt: f32, position: Vector2<f32>,
                     state: SpiderState) -> Vector2<f32> {
    match state {
        SpiderState::Resting(_)   => position,
        SpiderState::Wandering(_) => position, // TODO(rightfold): Move randomly.
        SpiderState::Attacking    => position, // TODO(rightfold): Move towards target.
    }
}

fn simulate_angle(dt: f32, angle: Rad<f32>,
                  rotation_state: RotationState) -> Rad<f32> {
    match rotation_state {
        RotationState::Zero => angle,
        RotationState::Clockwise => angle - Rad::from(ROTATION_SPEED * dt),
        RotationState::Counterclockwise => angle + Rad::from(ROTATION_SPEED * dt),
    }
}

fn simulate_state<R>(rng: &mut R, dt: f32, state: SpiderState) -> SpiderState
    where R: Rng {
    match state {
        // When a spider is attacking, it should keep attacking.
        SpiderState::Attacking => SpiderState::Attacking,

        // A resting or wandering spider may decide to start doing the other
        // thing from now on.
        SpiderState::Resting(_) | SpiderState::Wandering(_) => {
            let chance = (STATE_CHANGE_CHANCE as f32 / dt) as u32;
            let change_state = rng.gen_weighted_bool(chance);
            if change_state {
                let rotation = rng.gen();
                match rng.gen_range(0, 2) {
                    0 => SpiderState::Resting(rotation),
                    _ => SpiderState::Wandering(rotation),
                }
            } else {
                state
            }
        },
    }
}
