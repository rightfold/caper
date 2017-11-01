use cgmath::{Matrix4, Vector2};
use rand::Rng;

pub trait MonsterSet {
    type Id: Copy;

    fn ids(&self) -> &[Self::Id];

    fn spawn<R: Rng>(&mut self, rng: &mut R,
                     position: Vector2<f32>) -> Self::Id;
    fn despawn(&mut self, Self::Id);

    fn draw(&self, pmat: Matrix4<f32>, vmat: Matrix4<f32>, mmat: Matrix4<f32>,
            light_position: Vector2<f32>);
}

pub trait Mortal {
    fn healths(&self) -> &[f32];
}

pub mod gas_spore;
pub mod spider;
