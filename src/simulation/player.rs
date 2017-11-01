use cgmath::{InnerSpace, MetricSpace, Vector2, Zero};
use rand::Rng;

use chance::gen_range_base;
use input::Input;
use world::World;
use world::player::*;

const MOVEMENT_SPEED: f32 = 4.0;
const ATTACK_SPEED: f32 = 5.0;
const ATTACK_DAMAGE_BASE: f32 = 4.0;
const ATTACK_DAMAGE_RANGE: (f32, f32) = (-2.0, 1.0);
const ATTACK_RANGE: f32 = 1.0;
const ATTACK_KNOCK_BACK: f32 = 3.0;

pub fn simulate<R: Rng>(world: &mut World, input: &Input, rng: &mut R,
                        dt: f32) {
    simulate_move(&mut world.player, input, dt);
    simulate_attack_state(world, input, rng, dt);
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

fn simulate_attack_state<R: Rng>(world: &mut World, input: &Input, rng: &mut R,
                                 dt: f32) {
    if !input.attack {
        world.player.holding_attack = false;
    }
    match world.player.attack_state {
        AttackState::NotAttacking =>
            if input.attack && !world.player.holding_attack {
                world.player.attack_state = AttackState::Attacking(0.0);
                world.player.holding_attack = true;
                simulate_attack(world, rng);
            },
        AttackState::Attacking(since) if since > 1.0 =>
            world.player.attack_state = AttackState::NotAttacking,
        AttackState::Attacking(ref mut since) =>
            *since += ATTACK_SPEED * dt,
    }
}

fn simulate_attack<R: Rng>(world: &mut World, rng: &mut R) {
    let velocities = soa_array!(world.spiders, mut velocities).iter_mut();
    let positions  = soa_array!(world.spiders, positions).iter();
    let healths    = soa_array!(world.spiders, mut healths).iter_mut();
    for (velocity, (position, health)) in velocities.zip(positions.zip(healths)) {
        if world.player.position.distance(*position) < ATTACK_RANGE {
            let damage = gen_range_base(rng, ATTACK_DAMAGE_BASE,
                                        ATTACK_DAMAGE_RANGE);
            *health -= damage;
            world.damage_indicators.spawn(*position, damage);

            *velocity += (*position - world.player.position)
                .normalize_to(ATTACK_KNOCK_BACK);
        }
    }
}
