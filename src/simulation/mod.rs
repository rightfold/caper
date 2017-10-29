use rand::Rng;

use input::Input;
use world::World;
use world::monster;

pub fn simulate<R: Rng>(world: &mut World, input: &Input, rng: &mut R, dt: f32) {
    player::simulate(world, input, dt);

    spider::simulate(&mut world.spiders, rng, dt);
    // gas_spore::simulate(world, rng, dt);

    monster::despawn_dead(&mut world.spiders);
    monster::despawn_dead(&mut world.gas_spores);
}

pub mod player;
pub mod spider;
// pub mod gas_spore;
