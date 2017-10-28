use cgmath::{InnerSpace, Zero};

use input::Input;
use world::entity::catalog::player::*;

const MOVEMENT_SPEED: f32 = 0.004;
const ATTACK_INTERVAL: f32 = 400.0;
const SWORD_SWING_SPEED: f32 = 0.005;

pub struct SimulationState {
    since_attack: f32,
    attack_released: bool,
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState{
            since_attack: 0.0,
            attack_released: true,
        }
    }

    pub fn simulate(&mut self, dt: f32, input: &Input, player: &mut Player) {
        Self::simulate_move(dt, input, player);
        self.simulate_attack(dt, input, player);
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

    fn simulate_attack(&mut self, dt: f32, input: &Input, player: &mut Player) {
        self.since_attack += dt;

        if !input.attack {
            self.attack_released = true;
        }

        match player.sword_state {
            SwordState::Carrying =>
                if input.attack && self.may_attack() {
                    player.sword_state = SwordState::Swinging(0.0);
                    self.since_attack = 0.0;
                },
            SwordState::Swinging(mut t) => {
                t += SWORD_SWING_SPEED * dt;
                player.sword_state = if t < 1.0 {
                    self.attack_released = false;
                    SwordState::Swinging(t)
                } else {
                    SwordState::Carrying
                };
            },
        };
    }

    fn may_attack(&self) -> bool {
        self.attack_released && self.since_attack > ATTACK_INTERVAL
    }
}
