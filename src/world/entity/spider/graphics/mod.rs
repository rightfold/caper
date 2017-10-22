use cgmath::{Matrix4, Vector2, Vector3, Rad};
use gfx;

use graphics::{ColorFormat, DepthFormat};
use graphics::obj::Obj;
use world::entity::spider::SpiderSet;

gfx_defines! {
    vertex Vertex {
        position: [f32; 3] = "position",
    }

    constant Transform {
        world_transform: [[f32; 4]; 4] = "world_transform",
        model_transform: [[f32; 4]; 4] = "model_transform",
    }

    pipeline pipeline {
        vertex_buffer: gfx::VertexBuffer<Vertex> = (),
        transform_buffer: gfx::ConstantBuffer<Transform> = "Transform",
        render_target: gfx::RenderTarget<ColorFormat> = "target",
        depth_target: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl From<Vector3<f32>> for Vertex {
    fn from(other: Vector3<f32>) -> Self {
        Vertex{position: other.into()}
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
        depth_target: gfx::handle::DepthStencilView<R, DepthFormat>,
    ) -> Result<Self, gfx::PipelineStateError<String>>
        where F: gfx::traits::FactoryExt<R> {
        let pso = factory.create_pipeline_simple(
            include_bytes!("spider.vert"),
            include_bytes!("spider.frag"),
            pipeline::new(),
        )?;

        let body_model = Obj::read(include_str!(concat!(env!("OUT_DIR"), "/world/entity/spider/graphics/body.obj"))).unwrap();

        let (vertex_buffer, vertex_slice) =
            factory.create_vertex_buffer_with_slice(&body_model.vertices,
                                                    &body_model.indices[..]);

        let transform_buffer = factory.create_constant_buffer(1);

        Ok(DrawState{
            pso,
            vertex_slice,
            data: pipeline::Data{
                vertex_buffer,
                transform_buffer,
                render_target,
                depth_target,
            },
        })
    }

    pub fn draw<C>(&self, encoder: &mut gfx::Encoder<R, C>, base_transform: Matrix4<f32>, spiders: &SpiderSet) -> Result<(), gfx::UpdateError<usize>>
        where C: gfx::CommandBuffer<R> {
        let positions = spiders.positions().iter();
        let angles = spiders.angles().iter();
        for (position, angle) in positions.zip(angles) {
            self.draw_one(encoder, base_transform, *position, *angle)?;
        }
        Ok(())
    }

    fn draw_one<C>(&self, encoder: &mut gfx::Encoder<R, C>, world_transform: Matrix4<f32>, position: Vector2<f32>, angle: Rad<f32>) -> Result<(), gfx::UpdateError<usize>>
        where C: gfx::CommandBuffer<R> {
        let model_transform =
            Matrix4::from_translation(position.extend(0.0)) *
            Matrix4::from_angle_z(angle);
        encoder.update_buffer(&self.data.transform_buffer,
                              &[Transform{world_transform: world_transform.into(),
                                          model_transform: model_transform.into()}],
                              0)?;
        encoder.draw(&self.vertex_slice, &self.pso, &self.data);
        Ok(())
    }
}
