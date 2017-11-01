use cgmath::{InnerSpace, MetricSpace, Vector2, Zero};
use rand::Rng;

use chance::gen_range_base;
use input::Input;
use world::World;
use world::effect::damage_indicator::DamageIndicatorSet;
use world::player::*;

const MOVEMENT_SPEED: f32 = 4.0;
const ATTACK_SPEED: f32 = 5.0;
const ATTACK_DAMAGE_BASE: f32 = 4.0;
const ATTACK_DAMAGE_RANGE: (f32, f32) = (-2.0, 1.0);
const ATTACK_RANGE: f32 = 1.0;
const ATTACK_KNOCK_BACK: f32 = 3.0;

pub fn simulate<R: Rng>(world: &mut World, input: &Input, rng: &mut R,
                        dt: f32) {
    simulate_move(dt, input, &mut world.player);
    simulate_attack_state(rng, dt, input, world);
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

fn simulate_attack_state<R: Rng>(rng: &mut R, dt: f32, input: &Input,
                                 world: &mut World) {
    if !input.attack {
        world.player.holding_attack = false;
    }
    match world.player.attack_state {
        AttackState::NotAttacking =>
            if input.attack && !world.player.holding_attack {
                world.player.attack_state = AttackState::Attacking(0.0);
                world.player.holding_attack = true;
                simulate_attack(rng, world);
            },
        AttackState::Attacking(since) if since > 1.0 =>
            world.player.attack_state = AttackState::NotAttacking,
        AttackState::Attacking(ref mut since) =>
            *since += ATTACK_SPEED * dt,
    }
}

fn simulate_attack<R: Rng>(rng: &mut R, world: &mut World) {
    simulate_attack_impl(
        rng, &world.player, &mut world.damage_indicators,
        soa_array!(world.spiders, mut healths),
        soa_array!(world.spiders, positions),
        soa_array!(world.spiders, mut velocities),
    );
    simulate_attack_impl(
        rng, &world.player, &mut world.damage_indicators,
        soa_array!(world.gas_spores, mut healths),
        soa_array!(world.gas_spores, positions),
        soa_array!(world.gas_spores, mut velocities),
    );
}

fn simulate_attack_impl<R: Rng>(rng: &mut R, player: &Player,
                                damage_indicators: &mut DamageIndicatorSet,
                                healths: &mut [f32], positions: &[Vector2<f32>],
                                velocities: &mut [Vector2<f32>]) {
    let monsters =
        healths.iter_mut().zip(positions.iter().zip(velocities.iter_mut()));
    for (health, (position, velocity)) in monsters {
        if player.position.distance(*position) < ATTACK_RANGE {
            let damage = gen_range_base(rng, ATTACK_DAMAGE_BASE,
                                        ATTACK_DAMAGE_RANGE);
            *health -= damage;
            damage_indicators.spawn(*position, damage);

            *velocity += (*position - player.position)
                .normalize_to(ATTACK_KNOCK_BACK);
        }
    }
}
