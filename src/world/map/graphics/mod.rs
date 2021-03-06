use std::f32;

use cgmath::{Matrix4, Vector2};

use graphics;
use graphics::gl;
use world::map::{Map, Material};

#[derive(Debug)]
pub struct Graphics {
    program: gl::Program,
    vertex_array: gl::VertexArray,
    vertex_count: usize,
    _vertex_position_buffer: gl::Buffer<Vector2<f32>>,
    tile_material_buffer: gl::Buffer<Material>,
    tile_elevation_buffer: gl::Buffer<u8>,
}

impl Graphics {
    pub fn new() -> Self {
        let vertex_positions = Self::generate_model();

        let program = Self::new_program();

        let vertex_count = vertex_positions.len();

        let vertex_position_buffer = gl::Buffer::new();
        gl::named_buffer_data(&vertex_position_buffer, &vertex_positions,
                              gl::DataStoreUsage::StaticDraw);

        let tile_material_buffer = gl::Buffer::new();

        let tile_elevation_buffer = gl::Buffer::new();

        let vertex_array = gl::VertexArray::new();

        gl::bind_vertex_array(&vertex_array);

        gl::enable_vertex_attrib_array(0);
        gl::bind_buffer(gl::BufferTarget::ArrayBuffer, &vertex_position_buffer);
        gl::vertex_attrib_pointer::<Vector2<f32>>(0, false);

        gl::enable_vertex_attrib_array(1);
        gl::bind_buffer(gl::BufferTarget::ArrayBuffer, &tile_material_buffer);
        gl::vertex_attrib_i_pointer::<Material>(1);
        gl::vertex_attrib_divisor(1, 1);

        gl::enable_vertex_attrib_array(2);
        gl::bind_buffer(gl::BufferTarget::ArrayBuffer, &tile_elevation_buffer);
        gl::vertex_attrib_i_pointer::<u8>(2);
        gl::vertex_attrib_divisor(2, 1);

        Graphics{program, vertex_array, vertex_count,
                 _vertex_position_buffer: vertex_position_buffer,
                 tile_material_buffer, tile_elevation_buffer}
    }

    fn generate_model() -> [Vector2<f32>; 8] {
        const ANGLE: f32 = f32::consts::PI / 3.0;
        let mut vertex_positions = [Vector2::new(0.0, 0.0); 8];
        for i in 1 .. 7 {
            let (x, y) = (ANGLE * i as f32).sin_cos();
            vertex_positions[i] = Vector2::new(0.45 * x, 0.45 * y);
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

    pub fn draw(&self, map: &Map, pmat: Matrix4<f32>, vmat: Matrix4<f32>,
                mmat: Matrix4<f32>, light_position: Vector2<f32>) {
        gl::bind_vertex_array(&self.vertex_array);

        gl::draw_buffer(gl::ColorBuffer::Back);

        gl::use_program(&self.program);

        gl::uniform(0, pmat);
        gl::uniform(1, vmat);
        gl::uniform(2, mmat);
        gl::uniform(3, light_position);

        for (&sector_id, sector) in map.sectors.iter() {
            gl::named_buffer_data(&self.tile_material_buffer,
                                  &sector.materials[..],
                                  gl::DataStoreUsage::DynamicDraw);

            gl::named_buffer_data(&self.tile_elevation_buffer,
                                  &sector.elevations[..],
                                  gl::DataStoreUsage::DynamicDraw);

            gl::uniform(4, sector_id);

            gl::draw_arrays_instanced(gl::PrimitiveType::TriangleFan,
                                      self.vertex_count,
                                      sector.materials.len());
        }
    }
}
