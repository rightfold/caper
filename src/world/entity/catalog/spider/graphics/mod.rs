use cgmath::{Matrix4, Vector2, Vector3};

use graphics::gl;
use graphics::obj::Obj;
use world::entity::catalog::spider::SpiderSet;

pub struct DrawState {
    program: gl::Program,
    vertex_array: gl::VertexArray,
    _vertex_buffer: gl::Buffer,
    vertex_index_count: usize,
    vertex_index_buffer: gl::Buffer,
    positions_buffer: gl::Buffer,
    angles_buffer: gl::Buffer,
}

impl DrawState {
    pub fn new() -> Self {
        let model: Obj<Vector3<f32>> = Obj::read(include_str!(concat!(env!("OUT_DIR"), "/world/entity/catalog/spider/graphics/body.obj"))).unwrap();

        let program = Self::new_program();

        let vertex_buffer = gl::Buffer::new();
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer, &vertex_buffer);
        gl::buffer_data(gl::BufferBindingTarget::ArrayBuffer, &model.vertices,
                        gl::DataStoreUsage::StaticDraw);

        let vertex_index_buffer = gl::Buffer::new();
        gl::bind_buffer(gl::BufferBindingTarget::ElementArrayBuffer,
                        &vertex_index_buffer);
        gl::buffer_data(gl::BufferBindingTarget::ElementArrayBuffer,
                        &model.indices, gl::DataStoreUsage::StaticDraw);

        let positions_buffer = gl::Buffer::new();

        let angles_buffer = gl::Buffer::new();

        let vertex_array = gl::VertexArray::new();

        gl::bind_vertex_array(&vertex_array);

        gl::enable_vertex_attrib_array(0);
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer, &vertex_buffer);
        gl::vertex_attrib_pointer::<Vector3<f32>>(0, false);
        gl::vertex_attrib_divisor(0, 0);

        gl::enable_vertex_attrib_array(1);
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer, &positions_buffer);
        gl::vertex_attrib_pointer::<Vector2<f32>>(1, false);
        gl::vertex_attrib_divisor(1, 1);

        gl::enable_vertex_attrib_array(2);
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer, &angles_buffer);
        gl::vertex_attrib_pointer::<f32>(2, false);
        gl::vertex_attrib_divisor(2, 1);

        DrawState{program, vertex_array, _vertex_buffer: vertex_buffer,
                  vertex_index_count: model.indices.len(), vertex_index_buffer,
                  positions_buffer, angles_buffer}
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
        let _ = world_transform;

        gl::bind_vertex_array(&self.vertex_array);

        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer,
                        &self.positions_buffer);
        gl::buffer_data(gl::BufferBindingTarget::ArrayBuffer,
                        spiders.positions(), gl::DataStoreUsage::StreamDraw);

        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer,
                        &self.angles_buffer);
        gl::buffer_data(gl::BufferBindingTarget::ArrayBuffer,
                        spiders.angles(), gl::DataStoreUsage::StreamDraw);

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
