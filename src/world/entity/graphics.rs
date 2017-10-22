use cgmath::Matrix4;
use gfx;

use graphics::{ColorFormat, DepthFormat};
use world::entity::{Entities, player, spider};

pub struct DrawState<R> where R: gfx::Resources {
    player: player::graphics::DrawState<R>,
    spiders: spider::graphics::DrawState<R>,
}

impl<R> DrawState<R> where R: gfx::Resources {
    pub fn new<F>(
        factory: &mut F,
        render_target: gfx::handle::RenderTargetView<R, ColorFormat>,
        depth_target: gfx::handle::DepthStencilView<R, DepthFormat>,
    ) -> Result<Self, gfx::PipelineStateError<String>>
        where F: gfx::traits::FactoryExt<R> {
        Ok(DrawState{
            player: player::graphics::DrawState::new(factory, render_target.clone())?,
            spiders: spider::graphics::DrawState::new(factory, render_target, depth_target)?,
        })
    }

    pub fn draw<C>(&self, encoder: &mut gfx::Encoder<R, C>, world_transform: Matrix4<f32>, entities: &Entities) -> Result<(), gfx::UpdateError<usize>>
        where C: gfx::CommandBuffer<R> {
        self.player.draw(encoder, world_transform, &entities.player)?;
        self.spiders.draw(encoder, world_transform, &entities.spiders)?;
        Ok(())
    }
}
