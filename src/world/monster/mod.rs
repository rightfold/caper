use cgmath::{Matrix4, Vector2};
use rand::Rng;

pub trait MonsterSet {
    type Id: Copy;

    fn ids(&self) -> &[Self::Id];
    fn positions(&self) -> &[Vector2<f32>];
    fn healths(&self) -> &[f32];

    fn spawn<R: Rng>(&mut self, rng: &mut R,
                     position: Vector2<f32>) -> Self::Id;
    fn despawn(&mut self, Self::Id);

    fn draw(&self, pmat: Matrix4<f32>, vmat: Matrix4<f32>, mmat: Matrix4<f32>,
            light_position: Vector2<f32>);
}

pub fn despawn_dead<M: MonsterSet>(monsters: &mut M) {
    let dead = {
        let ids = monsters.ids().iter();
        let healths = monsters.healths().iter();
        ids.zip(healths)
            .filter(|&(_, &health)| health < 0.0)
            .map(|(&id, _)| id)
            .collect::<Vec<_>>()
    };
    for dead in dead {
        monsters.despawn(dead);
    }
}

pub mod gas_spore;
pub mod spider;
