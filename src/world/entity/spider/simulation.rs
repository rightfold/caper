use cgmath::{Rad, Vector2};
use rand::Rng;

use world::entity::spider::*;

pub fn simulate_one<R: Rng>(position: &mut Vector2<f32>, angle: &mut Rad<f32>, state: &mut SpiderState, rng: &mut R) {
    // TODO(rightfold): Update rotation state.

    *position = simulate_position(*position, *state);

    *angle = simulate_angle(*angle, state.rotation_state());

    let change_state = rng.gen_weighted_bool(STATE_CHANGE_CHANCE);
    *state = match *state {
        SpiderState::Resting(rotation) if change_state => SpiderState::Wandering(rotation),
        SpiderState::Wandering(rotation) if change_state => SpiderState::Resting(rotation),
        other => other,
    };
}

fn simulate_position(position: Vector2<f32>, state: SpiderState) -> Vector2<f32> {
    match state {
        SpiderState::Resting(_)   => position,
        SpiderState::Wandering(_) => position,
        SpiderState::Attacking    => position, // TODO(rightfold): Move towards target.
    }
}

fn simulate_angle(angle: Rad<f32>, rotation_state: RotationState) -> Rad<f32> {
    match rotation_state {
        RotationState::Zero => angle,
        RotationState::Clockwise => angle + Rad::from(ROTATION_SPEED),
        RotationState::Counterclockwise => angle - Rad::from(ROTATION_SPEED),
    }
}
