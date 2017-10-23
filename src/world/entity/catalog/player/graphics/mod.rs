use cgmath::Matrix4;

use world::entity::catalog::player::Player;

pub struct DrawState {
}

impl DrawState {
    pub fn new() -> Self {
        DrawState{}
    }

    pub fn draw(&self, world_transform: Matrix4<f32>, player: &Player) {
        let _ = world_transform;
        let _ = player;
    }
}
