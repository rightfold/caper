use std::f32;

use cgmath::{Matrix4, Vector2};

use graphics;
use graphics::gl;
use world::map::{Map, Material};

pub struct GraphicsState {
    program: gl::Program,
    vertex_array: gl::VertexArray,
    vertex_count: usize,
    _vertex_position_buffer: gl::Buffer<Vector2<f32>>,
    tile_material_buffer: gl::Buffer<Material>,
}

impl GraphicsState {
    pub fn new() -> Self {
        let vertex_positions = Self::generate_model();

        let program = Self::new_program();

        let vertex_count = vertex_positions.len();

        let vertex_position_buffer = gl::Buffer::new();
        gl::named_buffer_data(&vertex_position_buffer, &vertex_positions,
                              gl::DataStoreUsage::StaticDraw);

        let tile_material_buffer = gl::Buffer::new();

        let vertex_array = gl::VertexArray::new();

        gl::bind_vertex_array(&vertex_array);

        gl::enable_vertex_attrib_array(0);
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer,
                        &vertex_position_buffer);
        gl::vertex_attrib_pointer::<Vector2<f32>>(0, false);

        gl::enable_vertex_attrib_array(1);
        gl::bind_buffer(gl::BufferBindingTarget::ArrayBuffer,
                        &tile_material_buffer);
        gl::vertex_attrib_i_pointer::<Material>(1);
        gl::vertex_attrib_divisor(1, 1);

        GraphicsState{program, vertex_array, vertex_count,
                      _vertex_position_buffer: vertex_position_buffer,
                      tile_material_buffer}
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
        graphics::catalog::make_program(
            include_bytes!("map.vert"),
            include_bytes!("map.frag"),
        )
    }

    pub fn draw(&self, world_transform: Matrix4<f32>, map: &Map) {
        gl::bind_vertex_array(&self.vertex_array);

        gl::draw_buffer(gl::ColorBuffer::Back);

        gl::use_program(&self.program);

        gl::uniform(0, world_transform);

        for (&sector_id, sector) in map.sectors.iter() {
            gl::named_buffer_data(&self.tile_material_buffer,
                                  &sector.materials[..],
                                  gl::DataStoreUsage::DynamicDraw);

            gl::uniform(1, sector_id);

            gl::draw_arrays_instanced(gl::PrimitiveType::TriangleFan,
                                      self.vertex_count,
                                      sector.materials.len());
        }
    }
}
