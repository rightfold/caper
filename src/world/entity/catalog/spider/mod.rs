use cgmath::{Rad, Vector2};
use rand::{Rand, Rng};

use chance::gen_range_base;

const INITIAL_HEALTH_BASE: i16 = 20;
const INITIAL_HEALTH_CHANCE: (i16, i16) = (-5, 5);

entity_set!(
    SpiderSet,
    SpiderId,
    positions: Vector2<f32>,
    angles:    Rad<f32>,
    healths:   i16,
    actions:   Action,
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Resting(RotationAction),
    Wandering(RotationAction),
    Attacking,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RotationAction {
    Zero,
    Clockwise,
    Counterclockwise,
}

impl Rand for RotationAction {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 3) {
            0 => RotationAction::Zero,
            1 => RotationAction::Clockwise,
            _ => RotationAction::Counterclockwise,
        }
    }
}

impl SpiderSet {
    pub fn spawn<R: Rng>(&mut self, rng: &mut R, position: Vector2<f32>) -> SpiderId {
        entity_set_spawn_method_body!(
            self,
            positions: position,
            angles:    rng.gen(),
            healths:   gen_range_base(rng, INITIAL_HEALTH_BASE, INITIAL_HEALTH_CHANCE),
            actions:   Action::Wandering(RotationAction::Clockwise),
        )
    }
}

pub mod simulation;
pub mod graphics;
