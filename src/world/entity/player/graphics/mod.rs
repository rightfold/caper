use cgmath::Matrix4;
use gfx;

use graphics::ColorFormat;
use world::entity::player::Player;

gfx_defines! {
    vertex Vertex {
        position: [f32; 2] = "position",
    }

    constant Transform {
        world_transform: [[f32; 4]; 4] = "world_transform",
        model_transform: [[f32; 4]; 4] = "model_transform",
    }

    pipeline pipeline {
        vertex_buffer: gfx::VertexBuffer<Vertex> = (),
        transform_buffer: gfx::ConstantBuffer<Transform> = "Transform",
        render_target: gfx::RenderTarget<ColorFormat> = "target",
    }
}

pub struct DrawState<R> where R: gfx::Resources {
    pso: gfx::pso::PipelineState<R, pipeline::Meta>,
    vertex_slice: gfx::Slice<R>,
    data: pipeline::Data<R>,
}

impl<R> DrawState<R> where R: gfx::Resources {
    pub fn new<F>(
        factory: &mut F,
        render_target: gfx::handle::RenderTargetView<R, ColorFormat>,
    ) -> Result<Self, gfx::PipelineStateError<String>>
        where F: gfx::traits::FactoryExt<R> {
        let pso = factory.create_pipeline_simple(
            include_bytes!("player.vert"),
            include_bytes!("player.frag"),
            pipeline::new(),
        )?;

        let (vertex_buffer, vertex_slice) =
            factory.create_vertex_buffer_with_slice(&SQUARE, ());

        let transform_buffer = factory.create_constant_buffer(1);

        Ok(DrawState{
            pso,
            vertex_slice,
            data: pipeline::Data{
                vertex_buffer,
                transform_buffer,
                render_target,
            },
        })
    }

    pub fn draw<C>(&self, encoder: &mut gfx::Encoder<R, C>, world_transform: Matrix4<f32>, player: &Player) -> Result<(), gfx::UpdateError<usize>>
        where C: gfx::CommandBuffer<R> {
        let model_transform =
            Matrix4::from_translation(player.position.extend(0.0));
        encoder.update_buffer(&self.data.transform_buffer,
                              &[Transform{world_transform: world_transform.into(),
                                          model_transform: model_transform.into()}],
                              0)?;
        encoder.draw(&self.vertex_slice, &self.pso, &self.data);
        Ok(())
    }
}

const SQUARE: [Vertex; 6] = [
    Vertex{position: [-0.5, -0.5]},
    Vertex{position: [ 0.5, -0.5]},
    Vertex{position: [ 0.5,  0.5]},

    Vertex{position: [ 0.5,  0.5]},
    Vertex{position: [-0.5,  0.5]},
    Vertex{position: [-0.5, -0.5]},
];
