use cgmath::{Matrix4, Rad, Vector2, Vector3};

use graphics::gl;
use graphics::obj::Obj;
use world::entity::catalog::spider::SpiderSet;

pub struct DrawState {
    program: gl::Program,
    vertex_array: gl::VertexArray,
    _vertex_position_buffer: gl::Buffer<Vector3<f32>>,
    vertex_index_count: usize,
    vertex_index_buffer: gl::Buffer<u32>,
    model_position_buffer: gl::Buffer<Vector2<f32>>,
    model_angle_buffer: gl::Buffer<Rad<f32>>,
}

impl DrawState {
    pub fn new() -> Self {
        let model: Obj<Vector3<f32>> = Obj::read(include_str!(concat!(env!("OUT_DIR"), "/world/entity/catalog/spider/graphics/spider.obj"))).unwrap();

        let program = Self::new_program();

        let vertex_position_buffer = gl::Buffer::new();
        gl::named_buffer_data(&vertex_position_buffer, &model.vertex_positions,
                              gl::DataStoreUsage::StaticDraw);

        let vertex_index_count = model.vertex_indices.len();
        let vertex_index_buffer = gl::Buffer::new();
        gl::named_buffer_data(&vertex_index_buffer, &model.vertex_indices,
                              gl::DataStoreUsage::StaticDraw);

        let model_position_buffer = gl::Buffer::new();

        let model_angle_buffer = gl::Buffer::new();

        let vertex_array = gl::VertexArray::new();

        gl::bind_vertex_array(&vertex_array);

        gl::enable_vertex_attrib_array(0);
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer, &vertex_position_buffer);
        gl::vertex_attrib_pointer::<Vector3<f32>>(0, false);

        gl::enable_vertex_attrib_array(1);
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer, &model_position_buffer);
        gl::vertex_attrib_pointer::<Vector2<f32>>(1, false);
        gl::vertex_attrib_divisor(1, 1);

        gl::enable_vertex_attrib_array(2);
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer, &model_angle_buffer);
        gl::vertex_attrib_pointer::<f32>(2, false);
        gl::vertex_attrib_divisor(2, 1);

        DrawState{program, vertex_array,
                  _vertex_position_buffer: vertex_position_buffer,
                  vertex_index_count, vertex_index_buffer, model_position_buffer,
                  model_angle_buffer}
    }

    fn new_program() -> gl::Program {
        let vertex_shader = gl::Shader::new(gl::ShaderType::VertexShader);
        gl::shader_source(&vertex_shader, &[&include_bytes!("spider.vert")[..]]);
        gl::compile_shader(&vertex_shader);

        let fragment_shader = gl::Shader::new(gl::ShaderType::FragmentShader);
        gl::shader_source(&fragment_shader, &[&include_bytes!("spider.frag")[..]]);
        gl::compile_shader(&fragment_shader);

        let program = gl::Program::new();
        gl::attach_shader(&program, &vertex_shader);
        gl::attach_shader(&program, &fragment_shader);
        gl::link_program(&program);

        program
    }

    pub fn draw(&self, world_transform: Matrix4<f32>, spiders: &SpiderSet) {
        gl::bind_vertex_array(&self.vertex_array);

        gl::named_buffer_data(&self.model_position_buffer, spiders.positions(),
                              gl::DataStoreUsage::StreamDraw);

        gl::named_buffer_data(&self.model_angle_buffer, spiders.angles(),
                              gl::DataStoreUsage::StreamDraw);

        gl::bind_buffer(gl::BufferBindingTarget::ElementArrayBuffer,
                        &self.vertex_index_buffer);

        gl::draw_buffer(gl::ColorBuffer::Back);

        gl::use_program(&self.program);

        gl::uniform(0, world_transform);

        gl::draw_elements_instanced::<u32>(gl::PrimitiveType::Triangles,
                                           self.vertex_index_count,
                                           spiders.positions.len());
    }
}
