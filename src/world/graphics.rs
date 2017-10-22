use cgmath::{EuclideanSpace, Matrix4, Point3, Vector3};
use gfx;

use graphics::{ColorFormat, DepthFormat};
use world::{World, entity};

pub struct DrawState<R> where R: gfx::Resources {
    entities: entity::graphics::DrawState<R>,
}

impl<R> DrawState<R> where R: gfx::Resources {
    pub fn new<F>(
        factory: &mut F,
        render_target: gfx::handle::RenderTargetView<R, ColorFormat>,
        depth_target: gfx::handle::DepthStencilView<R, DepthFormat>,
    ) -> Result<Self, gfx::PipelineStateError<String>>
        where F: gfx::traits::FactoryExt<R> {
        Ok(DrawState{
            entities: entity::graphics::DrawState::new(factory, render_target, depth_target)?,
        })
    }

    pub fn draw<C>(&self, encoder: &mut gfx::Encoder<R, C>, projection_transform: Matrix4<f32>, world: &World) -> Result<(), gfx::UpdateError<usize>>
        where C: gfx::CommandBuffer<R> {
        let view_transform = Matrix4::look_at(
            Point3::from_vec(world.camera_position()),
            Point3::from_vec(world.player_position().extend(0.0)),
            Vector3::new(0.0, 0.0, 1.0),
        );
        let world_transform = projection_transform * view_transform;

        self.entities.draw(encoder, world_transform, &world.entities)?;

        Ok(())
    }
}
