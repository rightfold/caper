use cgmath::{Rad, Vector2};
use rand::{Rand, Rng};

use chance::gen_range_base;
use graphics::obj::Obj;

const INITIAL_HEALTH_BASE: f32 = 20.0;
const INITIAL_HEALTH_CHANCE: (f32, f32) = (-5.0, 5.0);

monster_set! {
    SpiderSet,

    SpiderId,

    attribute healths: f32,
    attribute actions: Action,

    visible(location = 2) positions: Vector2<f32>,
    visible(location = 3) angles:    Rad<f32>,

    state since_action_change: f32 = 0.0,

    spawn(rng, position) {
        healths: gen_range_base(rng, INITIAL_HEALTH_BASE, INITIAL_HEALTH_CHANCE),
        actions: Action::Wandering(RotationAction::Clockwise),

        positions: position,
        angles:    rng.gen(),
    },

    graphics {
        model: Obj::read(include_str!(concat!(env!("OUT_DIR"), "/world/monster/spider/graphics/spider.obj"))).unwrap(),

        vertex_shader:   include_bytes!("graphics/spider.vert"),
        fragment_shader: include_bytes!("graphics/spider.frag"),
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Resting(RotationAction),
    Wandering(RotationAction),
    Attacking,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RotationAction {
    Zero,
    Clockwise,
    Counterclockwise,
}

impl Rand for RotationAction {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 3) {
            0 => RotationAction::Zero,
            1 => RotationAction::Clockwise,
            _ => RotationAction::Counterclockwise,
        }
    }
}
