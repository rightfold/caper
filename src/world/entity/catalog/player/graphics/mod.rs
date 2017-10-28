use cgmath::{Matrix4, Vector2, Vector3};

use graphics;
use graphics::gl;
use graphics::obj::Obj;
use world::entity::catalog::player::Player;

pub struct GraphicsState {
    program: gl::Program,
    vertex_array: gl::VertexArray,
    _vertex_position_buffer: gl::Buffer<Vector3<f32>>,
    _vertex_normal_buffer: gl::Buffer<Vector3<f32>>,
    vertex_index_count: usize,
    vertex_index_buffer: gl::Buffer<u32>,
}

impl GraphicsState {
    pub fn new() -> Self {
        let model: Obj<Vector3<f32>, Vector3<f32>> =
            Obj::read(include_str!(concat!(env!("OUT_DIR"),
                                           "/world/entity/catalog/player/graphics/player.obj"))).unwrap();

        let program = Self::new_program();

        let vertex_position_buffer = gl::Buffer::new();
        gl::named_buffer_data(&vertex_position_buffer, &model.vertex_positions,
                              gl::DataStoreUsage::StaticDraw);

        let vertex_normal_buffer = gl::Buffer::new();
        gl::named_buffer_data(&vertex_normal_buffer, &model.vertex_normals,
                              gl::DataStoreUsage::StaticDraw);

        let vertex_index_count = model.vertex_indices.len();
        let vertex_index_buffer = gl::Buffer::new();
        gl::named_buffer_data(&vertex_index_buffer, &model.vertex_indices,
                              gl::DataStoreUsage::StaticDraw);

        let vertex_array = gl::VertexArray::new();

        gl::bind_vertex_array(&vertex_array);

        gl::enable_vertex_attrib_array(0);
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer,
                        &vertex_position_buffer);
        gl::vertex_attrib_pointer::<Vector3<f32>>(0, false);

        gl::enable_vertex_attrib_array(1);
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer,
                        &vertex_normal_buffer);
        gl::vertex_attrib_pointer::<Vector3<f32>>(1, false);

        GraphicsState{program, vertex_array,
                      _vertex_position_buffer: vertex_position_buffer,
                      _vertex_normal_buffer: vertex_normal_buffer,
                      vertex_index_count, vertex_index_buffer}
    }

    fn new_program() -> gl::Program {
        graphics::catalog::make_program(
            include_bytes!("player.vert"),
            include_bytes!("player.frag"),
        )
    }

    pub fn draw(&self, world_transform: Matrix4<f32>,
                light_position: Vector2<f32>, player: &Player) {
        let model_transform =
            Matrix4::from_translation(player.position.extend(0.0))
            * Matrix4::from_angle_z(player.angle);

        gl::bind_vertex_array(&self.vertex_array);

        gl::bind_buffer(gl::BufferBindingTarget::ElementArrayBuffer,
                        &self.vertex_index_buffer);

        gl::draw_buffer(gl::ColorBuffer::Back);

        gl::use_program(&self.program);

        gl::uniform(0, world_transform);
        gl::uniform(1, model_transform);
        gl::uniform(2, light_position);

        gl::draw_elements::<u32>(gl::PrimitiveType::Triangles,
                                 self.vertex_index_count);
    }
}
