use cgmath::{Rad, Vector2, Zero};

use chance::gen_range_base;
use graphics::obj::Obj;
use timing::Since;

const INITIAL_HEALTH_BASE: f32 = 20.0;
const INITIAL_HEALTH_CHANCE: (f32, f32) = (-5.0, 5.0);

monster_set! {
    SpiderSet,

    SpiderId,

    attribute velocities: Vector2<f32>,
    attribute targets: Vector2<f32>,
    attribute healths: f32,

    visible(location = 2) positions: Vector2<f32>,
    visible(location = 3) angles:    Rad<f32>,

    state since_target_changes: Since = Since::new(),

    spawn(rng, position) {
        velocities: Vector2::zero(),

        targets: rng.gen(),
        healths: gen_range_base(rng, INITIAL_HEALTH_BASE, INITIAL_HEALTH_CHANCE),

        positions: position,
        angles:    rng.gen(),
    },

    graphics {
        model: Obj::read(include_str!(concat!(env!("OUT_DIR"), "/world/monster/spider/graphics/spider.obj"))).unwrap(),

        vertex_shader:   include_bytes!("graphics/spider.vert"),
        fragment_shader: include_bytes!("graphics/spider.frag"),
    },
}
