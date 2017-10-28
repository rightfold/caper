use cgmath::{InnerSpace, Zero};

use input::Input;
use world::entity::catalog::player::*;

const MOVEMENT_SPEED: f32 = 0.004;

pub struct SimulationState {
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState{}
    }

    pub fn simulate(&mut self, dt: f32, input: &Input, player: &mut Player) {
        let dposition = MOVEMENT_SPEED * dt * Vector2::new(
            -(input.move_west  as i32 as f32) + (input.move_east  as i32 as f32),
             (input.move_north as i32 as f32) - (input.move_south as i32 as f32),
        );
        if !dposition.is_zero() {
            player.position += dposition;
            player.angle = Vector2::new(1.0, 0.0).angle(dposition);
        }
    }
}
