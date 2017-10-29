use cgmath::{Rad, Vector2};

use chance::gen_range_base;
use graphics::obj::Obj;

const INITIAL_HEALTH_BASE: f32 = 10.0;
const INITIAL_HEALTH_CHANCE: (f32, f32) = (-3.0, 5.0);

const INITIAL_ALTITUDE_BASE: f32 = 0.7;
const INITIAL_ALTITUDE_CHANCE: (f32, f32) = (-0.15, 0.15);

monster_set! {
    GasSporeSet,

    GasSporeId,

    attribute healths: f32,

    visible(location = 2) positions: Vector2<f32>,
    visible(location = 3) angles: Rad<f32>,
    visible(location = 4) altitudes: f32,

    spawn(rng, position) {
        healths:   gen_range_base(rng, INITIAL_HEALTH_BASE, INITIAL_HEALTH_CHANCE),

        positions: position,
        angles:    rng.gen(),
        altitudes: gen_range_base(rng, INITIAL_ALTITUDE_BASE, INITIAL_ALTITUDE_CHANCE),
    },

    graphics {
        model: Obj::read(include_str!(concat!(env!("OUT_DIR"), "/world/monster/gas_spore/graphics/gas_spore.obj"))).unwrap(),

        vertex_shader:   include_bytes!("graphics/gas_spore.vert"),
        fragment_shader: include_bytes!("graphics/gas_spore.frag"),
    },
}
