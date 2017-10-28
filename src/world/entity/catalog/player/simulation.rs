use cgmath::{InnerSpace, MetricSpace, Zero};

use input::Input;
use world::World;
use world::entity::catalog::player::*;

const MOVEMENT_SPEED: f32 = 0.004;
const ATTACK_INTERVAL: f32 = 400.0;
const ATTACK_SPEED: f32 = 0.005;
const ATTACK_DAMAGE: f32 = 0.01;
const ATTACK_RANGE: f32 = 1.0;

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

    pub fn simulate(&mut self, dt: f32, input: &Input, world: &mut World) {
        Self::simulate_move(dt, input, &mut world.player);
        self.simulate_attack_state(dt, input, &mut world.player);
        Self::simulate_attack(dt, world)
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

    fn simulate_attack_state(&mut self, dt: f32, input: &Input, player: &mut Player) {
        self.since_attack += dt;

        if !input.attack {
            self.attack_released = true;
        }

        let may_attack = self.attack_released && self.since_attack > ATTACK_INTERVAL;

        match player.attack_state {
            None =>
                if input.attack && may_attack {
                    player.attack_state = Some(AttackState{progress: 0.0});
                    self.since_attack = 0.0;
                },
            Some(AttackState{progress}) => {
                let new_progress = progress + ATTACK_SPEED * dt;
                if new_progress > 1.0 {
                    player.attack_state = None;
                } else {
                    self.attack_released = false;
                    player.attack_state = Some(AttackState{progress: new_progress});
                }
            },
        };
    }

    fn simulate_attack(dt: f32, world: &mut World) {
        if world.player.attack_state.is_none() {
            return;
        }

        let spider_positions = entity_field!(world.spiders, positions).iter();
        let spider_healths = entity_field!(world.spiders, mut healths).iter_mut();
        for (&position, health) in spider_positions.zip(spider_healths) {
            if world.player.position.distance(position) < ATTACK_RANGE {
                *health -= dt * ATTACK_DAMAGE;
            }
        }
    }
}
