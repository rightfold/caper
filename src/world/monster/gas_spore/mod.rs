use cgmath::{Rad, Vector2, Zero};

use chance::gen_range_base;
use graphics::obj::Obj;
use timing::Since;
use world::monster::Mortal;

const INITIAL_HEALTH_BASE: f32 = 10.0;
const INITIAL_HEALTH_CHANCE: (f32, f32) = (-3.0, 5.0);

monster_set! {
    GasSporeSet,

    GasSporeId,

    attribute velocities: Vector2<f32>,
    attribute targets: Vector2<f32>,
    attribute healths: f32,

    visible(location = 2) positions:        Vector2<f32>,
    visible(location = 3) angles:           Rad<f32>,
    visible(location = 4) altitude_arcsins: f32,

    state since_target_changes: Since = Since::new(),

    spawn(rng, position) {
        velocities: Vector2::zero(),
        targets: rng.gen(),
        healths: gen_range_base(rng, INITIAL_HEALTH_BASE, INITIAL_HEALTH_CHANCE),

        positions:        position,
        angles:           rng.gen(),
        altitude_arcsins: rng.gen_range(-1.0, 1.0),
    },

    graphics {
        model: Obj::read(include_str!(concat!(env!("OUT_DIR"), "/world/monster/gas_spore/graphics/gas_spore.obj"))).unwrap(),

        vertex_shader:   include_bytes!("graphics/gas_spore.vert"),
        fragment_shader: include_bytes!("graphics/gas_spore.frag"),
    },
}

impl Mortal for GasSporeSet {
    fn healths(&self) -> &[f32] {
        soa_array!(self, healths)
    }
}
