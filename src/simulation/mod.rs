use rand::Rng;

use input::Input;
use world::World;
use world::monster;

const CAMERA_DEAD_ZONE_N: f32 = 3.0;
const CAMERA_DEAD_ZONE_S: f32 = 1.5;
const CAMERA_DEAD_ZONE_W: f32 = 4.5;
const CAMERA_DEAD_ZONE_E: f32 = 4.5;

pub fn simulate<R: Rng>(world: &mut World, input: &Input, rng: &mut R, dt: f32) {
    player::simulate(world, input, dt);

    spider::simulate(&mut world.spiders, rng, dt);
    gas_spore::simulate(&mut world.gas_spores, rng, dt);

    monster::despawn_dead(&mut world.spiders);
    monster::despawn_dead(&mut world.gas_spores);

    if world.player.position.x - world.camera_target.x > CAMERA_DEAD_ZONE_E {
        world.camera_target.x = world.player.position.x - CAMERA_DEAD_ZONE_E;
    } else if world.player.position.x - world.camera_target.x < -CAMERA_DEAD_ZONE_W {
        world.camera_target.x = world.player.position.x + CAMERA_DEAD_ZONE_W;
    }
    if world.player.position.y - world.camera_target.y > CAMERA_DEAD_ZONE_N {
        world.camera_target.y = world.player.position.y - CAMERA_DEAD_ZONE_N;
    } else if world.player.position.y - world.camera_target.y < -CAMERA_DEAD_ZONE_S {
        world.camera_target.y = world.player.position.y + CAMERA_DEAD_ZONE_S;
    }
}

pub mod player;
pub mod spider;
pub mod gas_spore;
