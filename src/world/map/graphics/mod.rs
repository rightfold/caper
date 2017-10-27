use std::f32;

use cgmath::{Matrix4, Vector2};

use graphics::gl;
use world::map::Map;

pub struct GraphicsState {
    program: gl::Program,
    vertex_array: gl::VertexArray,
    vertex_count: usize,
    _vertex_position_buffer: gl::Buffer<Vector2<f32>>,
}

impl GraphicsState {
    pub fn new() -> Self {
        let vertex_positions = Self::generate_model();

        let program = Self::new_program();

        let vertex_count = vertex_positions.len();

        let vertex_position_buffer = gl::Buffer::new();
        gl::named_buffer_data(&vertex_position_buffer, &vertex_positions,
                              gl::DataStoreUsage::StaticDraw);

        let vertex_array = gl::VertexArray::new();

        gl::bind_vertex_array(&vertex_array);

        gl::enable_vertex_attrib_array(0);
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer,
                        &vertex_position_buffer);
        gl::vertex_attrib_pointer::<Vector2<f32>>(0, false);

        GraphicsState{program, vertex_array, vertex_count,
                      _vertex_position_buffer: vertex_position_buffer}
    }

    fn generate_model() -> [Vector2<f32>; 8] {
        const ANGLE: f32 = f32::consts::PI / 3.0;
        let mut vertex_positions = [Vector2::new(0.0, 0.0); 8];
        for i in 1 .. 7 {
            let (x, y) = (ANGLE * i as f32).sin_cos();
            vertex_positions[i] = Vector2::new(0.9 * x, 0.9 * y);
        }
        vertex_positions[7] = vertex_positions[1];
        vertex_positions
    }

    fn new_program() -> gl::Program {
        let vertex_shader = gl::Shader::new(gl::ShaderType::VertexShader);
        gl::shader_source(&vertex_shader, &[&include_bytes!("map.vert")[..]]);
        gl::compile_shader(&vertex_shader);

        let fragment_shader = gl::Shader::new(gl::ShaderType::FragmentShader);
        gl::shader_source(&fragment_shader, &[&include_bytes!("map.frag")[..]]);
        gl::compile_shader(&fragment_shader);

        let program = gl::Program::new();
        gl::attach_shader(&program, &vertex_shader);
        gl::attach_shader(&program, &fragment_shader);
        gl::link_program(&program);

        program
    }

    pub fn draw(&self, world_transform: Matrix4<f32>, map: &Map) {
        gl::bind_vertex_array(&self.vertex_array);

        gl::draw_buffer(gl::ColorBuffer::Back);

        gl::use_program(&self.program);

        gl::uniform(0, world_transform);

        for (&sector_id, _) in map.sectors.iter() {
            gl::uniform(1, sector_id);

            gl::draw_arrays_instanced(gl::PrimitiveType::TriangleFan,
                                      self.vertex_count, 8 * 8);
        }
    }
}
