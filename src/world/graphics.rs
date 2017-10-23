use cgmath::{EuclideanSpace, Matrix4, Point3, Vector3};

use world::{World, entity};

pub struct DrawState {
    player: entity::catalog::player::graphics::DrawState,

    spiders: entity::catalog::spider::graphics::DrawState,
}

impl DrawState {
    pub fn new() -> Self {
        DrawState{
            player: entity::catalog::player::graphics::DrawState::new(),

            spiders: entity::catalog::spider::graphics::DrawState::new(),
        }
    }

    pub fn draw(&self, projection_transform: Matrix4<f32>, world: &World) {
        let view_transform = Matrix4::look_at(
            Point3::from_vec(world.camera_position()),
            Point3::from_vec(world.player_position().extend(0.0)),
            Vector3::new(0.0, 0.0, 1.0),
        );
        let world_transform = projection_transform * view_transform;

        self.player.draw(world_transform, &world.player);

        self.spiders.draw(world_transform, &world.spiders);
    }
}
