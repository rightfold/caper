use cgmath::{Angle, Rad, Vector2};
use rand::Rng;

use world::World;
use world::monster::catalog::spider::*;

const MOVEMENT_SPEED: f32 = 0.0006;
const ROTATION_SPEED: Rad<f32> = Rad(0.0015);
const ACTION_CHANGE_INTERVAL: f32 = 2000.0;
const ACTION_CHANGE_CHANCE: u32 = 5;

pub struct SimulationState {
    since_action_change: f32,
}

impl SimulationState {
    pub fn new() -> Self {
        let since_action_change = 0.0;
        SimulationState{since_action_change}
    }

    pub fn simulate<R: Rng>(&mut self, rng: &mut R, dt: f32, world: &mut World) {
        Self::simulate_movements(dt, &mut world.spiders);
        Self::simulate_rotations(dt, &mut world.spiders);
        self.simulate_action_changes(rng, dt, &mut world.spiders);
    }

    fn simulate_movements(dt: f32, spiders: &mut SpiderSet) {
        let positions = monster_field!(spiders, mut positions).iter_mut();
        let angles = monster_field!(spiders, mut angles).iter();
        let actions = monster_field!(spiders, mut actions).iter();
        for (position, (angle, action)) in positions.zip(angles.zip(actions)) {
            *position = Self::simulate_movement(dt, *position, *angle, action);
        }
    }

    fn simulate_movement(dt: f32, position: Vector2<f32>, angle: Rad<f32>,
                         action: &Action) -> Vector2<f32> {
        match action {
            &Action::Resting(_) => position,
            &Action::Wandering(_) => {
                let (dy_unit, dx_unit) = angle.sin_cos();
                position + MOVEMENT_SPEED * dt * Vector2::new(dx_unit, dy_unit)
            },
            &Action::Attacking => unimplemented!(),
        }
    }

    fn simulate_rotations(dt: f32, spiders: &mut SpiderSet) {
        let angles = monster_field!(spiders, mut angles).iter_mut();
        let actions = monster_field!(spiders, mut actions).iter_mut();
        for (angle, action) in angles.zip(actions) {
            *angle = Self::simulate_rotation(dt, *angle, action);
        }
    }

    fn simulate_rotation(dt: f32, angle: Rad<f32>, action: &Action) -> Rad<f32> {
        let rotation_action = match action {
            &Action::Resting(rotation_action) => rotation_action,
            &Action::Wandering(rotation_action) => rotation_action,
            &Action::Attacking => unimplemented!(),
        };
        let rotation_factor = match rotation_action {
            RotationAction::Zero => 0.0,
            RotationAction::Clockwise => -1.0,
            RotationAction::Counterclockwise => 1.0,
        };
        angle + ROTATION_SPEED * dt * rotation_factor
    }

    fn simulate_action_changes<R: Rng>(&mut self, rng: &mut R, dt: f32,
                                       spiders: &mut SpiderSet) {
        self.since_action_change += dt;
        while self.since_action_change > ACTION_CHANGE_INTERVAL {
            self.since_action_change -= ACTION_CHANGE_INTERVAL;
            for action in monster_field!(spiders, mut actions).iter_mut() {
                *action = Self::simulate_action_change(rng, action);
            }
        }
    }

    fn simulate_action_change<R: Rng>(rng: &mut R, action: &Action) -> Action {
        let mut change_action = |next_action: fn(RotationAction) -> Action| {
            let chance = rng.gen_weighted_bool(ACTION_CHANGE_CHANCE);
            if chance { next_action(rng.gen()) } else { *action }
        };
        match action {
            &Action::Resting(_) => change_action(Action::Wandering),
            &Action::Wandering(_) => change_action(Action::Resting),
            &Action::Attacking => Action::Attacking,
        }
    }
}