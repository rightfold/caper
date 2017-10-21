use cgmath::{Deg, Point2, Rad};
use rand::Rng;

use chance::gen_range_base;

const INITIAL_HEALTH_BASE: i16 = 20;
const INITIAL_HEALTH_CHANCE: (i16, i16) = (-5, 5);

const RESTING_CHANCE: u32 = 120;
const WANDERING_CHANCE: u32 = 120;

const ROTATION_SPEED: Deg<f32> = Deg(1.0);

entity_set!(
    SpiderSet,
    SpiderId,
    positions: positions_mut: Point2<f32>,
    angles:    angles_mut:    Rad<f32>,
    healths:   healths_mut:   i16,
    states:    states_mut:    SpiderState,
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpiderState {
    Resting(RotationState),
    Wandering(RotationState),
    Attacking,
}

impl SpiderState {
    pub fn rotation_state(&self) -> RotationState {
        match self {
            &SpiderState::Resting(r) => r,
            &SpiderState::Wandering(r) => r,
            &SpiderState::Attacking => RotationState::Zero,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RotationState {
    Zero,
    Clockwise,
    Counterclockwise,
}

impl SpiderSet {
    pub fn spawn<R: Rng>(&mut self, rng: &mut R, position: Point2<f32>) -> SpiderId {
        entity_set_spawn_method_body!(
            self,
            positions: position,
            angles:    rng.gen(),
            healths:   gen_range_base(rng, INITIAL_HEALTH_BASE, INITIAL_HEALTH_CHANCE),
            states:    SpiderState::Resting(RotationState::Zero),
        )
    }

    pub fn simulate<R: Rng>(&mut self, rng: &mut R) {
        let positions = self.positions.iter_mut();
        let angles = self.angles.iter_mut();
        let states = self.states.iter_mut();
        for (position, (angle, state)) in positions.zip(angles.zip(states)) {
            simulate(position, angle, state, rng);
        }
    }
}

fn simulate<R: Rng>(position: &mut Point2<f32>, angle: &mut Rad<f32>, state: &mut SpiderState, rng: &mut R) {
    *position = simulate_position(*position, *state);

    *angle = simulate_angle(*angle, state.rotation_state());

    let start_wandering = rng.gen_weighted_bool(WANDERING_CHANCE);
    let start_resting = rng.gen_weighted_bool(RESTING_CHANCE);
    *state = match *state {
        SpiderState::Resting(rotation) if start_wandering => SpiderState::Wandering(rotation),
        SpiderState::Wandering(rotation) if start_resting => SpiderState::Resting(rotation),
        other => other,
    };
}

fn simulate_position(position: Point2<f32>, state: SpiderState) -> Point2<f32> {
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
