use cgmath::{EuclideanSpace, Matrix4, Point3, Vector3};

use world::{World, entity, map};

pub struct GraphicsState {
    map: map::graphics::GraphicsState,

    player: entity::catalog::player::graphics::GraphicsState,

    spiders: entity::catalog::spider::graphics::GraphicsState,
}

impl GraphicsState {
    pub fn new() -> Self {
        GraphicsState{
            map: map::graphics::GraphicsState::new(),

            player: entity::catalog::player::graphics::GraphicsState::new(),

            spiders: entity::catalog::spider::graphics::GraphicsState::new(),
        }
    }

    pub fn draw(&self, projection_transform: Matrix4<f32>, world: &World) {
        let view_transform = Matrix4::look_at(
            Point3::from_vec(world.camera_position()),
            Point3::from_vec(world.player_position().extend(0.0)),
            Vector3::new(0.0, 0.0, 1.0),
        );
        let world_transform = projection_transform * view_transform;
        let light_position = world.player_position();

        self.map.draw(world_transform, &world.map);

        self.player.draw(world_transform, light_position, &world.player);

        self.spiders.draw(world_transform, light_position, &world.spiders);
    }
}
