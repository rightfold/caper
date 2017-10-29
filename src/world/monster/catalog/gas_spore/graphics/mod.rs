use cgmath::{Matrix4, Vector2};

use world::monster::catalog::gas_spore::*;

pub struct GraphicsState {
}

impl GraphicsState {
    pub fn new() -> Self {
        GraphicsState{}
    }

    pub fn draw(&self, _pmat: Matrix4<f32>, _vmat: Matrix4<f32>,
                _mmat: Matrix4<f32>, _light_position: Vector2<f32>,
                _gas_spores: &GasSporeSet) {
    }
}
