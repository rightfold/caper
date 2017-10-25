use cgmath::{Deg, Rad, Vector2};
use rand::Rng;

use chance::gen_range_base;

pub const INITIAL_HEALTH_BASE: i16 = 20;
pub const INITIAL_HEALTH_CHANCE: (i16, i16) = (-5, 5);

pub const STATE_CHANGE_CHANCE: u32 = 120;

pub const ROTATION_SPEED: Deg<f32> = Deg(0.1);

entity_set!(
    SpiderSet,
    SpiderId,
    positions: positions_mut: Vector2<f32>,
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
    pub fn spawn<R: Rng>(&mut self, rng: &mut R, position: Vector2<f32>) -> SpiderId {
        entity_set_spawn_method_body!(
            self,
            positions: position,
            angles:    rng.gen(),
            healths:   gen_range_base(rng, INITIAL_HEALTH_BASE, INITIAL_HEALTH_CHANCE),
            states:    SpiderState::Resting(RotationState::Clockwise),
        )
    }

    pub fn simulate<R: Rng>(&mut self, rng: &mut R, dt: f32) {
        let positions = self.positions.iter_mut();
        let angles = self.angles.iter_mut();
        let states = self.states.iter_mut();
        for (position, (angle, state)) in positions.zip(angles.zip(states)) {
            simulation::simulate_one(rng, dt, position, angle, state);
        }
    }
}

pub mod simulation;
pub mod graphics;
