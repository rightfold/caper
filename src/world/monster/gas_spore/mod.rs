use cgmath::Vector2;

use chance::gen_range_base;
use graphics::obj::Obj;

const INITIAL_HEALTH_BASE: f32 = 10.0;
const INITIAL_HEALTH_CHANCE: (f32, f32) = (-3.0, 5.0);

monster_set! {
    GasSporeSet,

    GasSporeId,

    attribute positions: Vector2<f32>,
    attribute healths: f32,

    spawn(rng, position) {
        positions: position,
        healths:   gen_range_base(rng, INITIAL_HEALTH_BASE, INITIAL_HEALTH_CHANCE),
    },

    graphics {
        model: Obj::read(include_str!(concat!(env!("OUT_DIR"), "/world/monster/catalog/spider/graphics/spider.obj"))).unwrap(),

        vertex_shader:   include_bytes!("../spider/graphics/spider.vert"),
        fragment_shader: include_bytes!("../spider/graphics/spider.frag"),
    },
}
