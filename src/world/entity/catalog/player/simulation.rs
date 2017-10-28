use cgmath::{InnerSpace, Zero};

use input::Input;
use world::entity::catalog::player::*;

const MOVEMENT_SPEED: f32 = 0.004;
const SWORD_SWINGING_SPEED: f32 = 0.005;

pub struct SimulationState {
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState{}
    }

    pub fn simulate(&mut self, dt: f32, input: &Input, player: &mut Player) {
        Self::simulate_move(dt, input, player);
        Self::simulate_attack(dt, input, player);
    }

    fn simulate_move(dt: f32, input: &Input, player: &mut Player) {
        let dposition = MOVEMENT_SPEED * dt * Vector2::new(
            -(input.move_west  as i32 as f32) + (input.move_east  as i32 as f32),
             (input.move_north as i32 as f32) - (input.move_south as i32 as f32),
        );
        if !dposition.is_zero() {
            player.position += dposition;
            player.angle = Vector2::new(1.0, 0.0).angle(dposition);
        }
    }

    fn simulate_attack(dt: f32, input: &Input, player: &mut Player) {
        match player.sword_state {
            SwordState::Carrying => {
                if input.attack {
                    player.sword_state = SwordState::Swinging(0.0);
                } else {
                    player.sword_state = SwordState::Carrying;
                }
            },
            SwordState::Swinging(mut t) => {
                t += SWORD_SWINGING_SPEED * dt;
                player.sword_state = if t < 1.0 {
                    SwordState::Swinging(t)
                } else {
                    SwordState::Carrying
                };
            },
        };
    }
}
