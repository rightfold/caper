use cgmath::{Matrix4, Vector2};

use world::monster::catalog::gas_spore::*;

pub struct GraphicsState {
}

impl GraphicsState {
    pub fn new() -> Self {
        GraphicsState{}
    }

    pub fn draw(&self, pmat: Matrix4<f32>, vmat: Matrix4<f32>,
                mmat: Matrix4<f32>, light_position: Vector2<f32>,
                gas_spores: &GasSporeSet) {
    }
}
