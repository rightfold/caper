use cgmath::Vector2;
use rand::Rng;

use chance::gen_range_base;
use world::monster::MonsterSet;

const INITIAL_HEALTH_BASE: f32 = 10.0;
const INITIAL_HEALTH_CHANCE: (f32, f32) = (-3.0, 5.0);

monster_set!(
    GasSporeSet,
    GasSporeId,
    positions: Vector2<f32>,
    healths: f32,
);

impl MonsterSet for GasSporeSet {
    type Id = GasSporeId;
    fn ids(&self) -> &[GasSporeId] { self.ids() }
    fn positions(&self) -> &[Vector2<f32>] { monster_field!(self, positions) }
    fn healths(&self) -> &[f32] { monster_field!(self, healths) }
    fn despawn(&mut self, id: GasSporeId) { self.despawn(id) }
}

impl GasSporeSet {
    pub fn spawn<R: Rng>(&mut self, rng: &mut R, position: Vector2<f32>) -> GasSporeId {
        monster_set_spawn_method_body!(
            self,
            positions: position,
            healths:   gen_range_base(rng, INITIAL_HEALTH_BASE, INITIAL_HEALTH_CHANCE),
        )
    }
}

pub mod simulation;
pub mod graphics;