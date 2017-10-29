use cgmath::{InnerSpace, MetricSpace, Vector2, Zero};

use input::Input;
use world::World;
use world::player::*;

const MOVEMENT_SPEED: f32 = 0.004;
const ATTACK_INTERVAL: f32 = 400.0;
const ATTACK_SPEED: f32 = 0.005;
const ATTACK_DAMAGE: f32 = 0.01;
const ATTACK_RANGE: f32 = 1.0;

pub fn simulate(world: &mut World, input: &Input, dt: f32) {
    simulate_move(&mut world.player, input, dt);
    simulate_attack_state(world, input, dt);
    simulate_attack(world, dt);
}

fn simulate_move(player: &mut Player, input: &Input, dt: f32) {
    let dposition = MOVEMENT_SPEED * dt * Vector2::new(
        -(input.move_west  as i32 as f32) + (input.move_east  as i32 as f32),
        (input.move_north as i32 as f32) - (input.move_south as i32 as f32),
    );
    if !dposition.is_zero() {
        player.position += dposition;
        player.angle = Vector2::new(1.0, 0.0).angle(dposition);
    }
}

fn simulate_attack_state(world: &mut World, input: &Input, dt: f32) {
    world.player.state.since_attack += dt;

    if !input.attack {
        world.player.state.attack_released = true;
    }

    let may_attack =
        world.player.state.attack_released
        && world.player.state.since_attack > ATTACK_INTERVAL;

    match world.player.attack_state {
        None =>
            if input.attack && may_attack {
                world.player.attack_state = Some(AttackState{progress: 0.0});
                world.player.state.since_attack = 0.0;
            },
        Some(AttackState{progress}) => {
            let new_progress = progress + ATTACK_SPEED * dt;
            if new_progress > 1.0 {
                world.player.attack_state = None;
            } else {
                world.player.state.attack_released = false;
                world.player.attack_state = Some(AttackState{progress: new_progress});
            }
        },
    };
}

fn simulate_attack(world: &mut World, dt: f32) {
    if world.player.attack_state.is_none() {
        return;
    }

    {
        let positions = monster_field!(world.spiders, positions).iter();
        let healths = monster_field!(world.spiders, mut healths).iter_mut();
        for (&position, health) in positions.zip(healths) {
            if world.player.position.distance(position) < ATTACK_RANGE {
                *health -= dt * ATTACK_DAMAGE;
            }
        }
    }
}
