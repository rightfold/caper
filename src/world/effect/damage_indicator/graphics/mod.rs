use cgmath::{Matrix4, Vector2};
use gl as raw_gl;
use png;

use graphics;
use graphics::gl;
use world::effect::damage_indicator::DamageIndicatorSet;

#[derive(Debug)]
pub struct Graphics {
    program: gl::Program,
    vertex_array: gl::VertexArray,
    vertex_count: usize,
    _vertex_position_buffer: gl::Buffer<Vector2<f32>>,
    vertex_texcoord_buffer: gl::Buffer<Vector2<f32>>,
    positions_buffer: gl::Buffer<Vector2<f32>>,
    ages_buffer: gl::Buffer<f32>,
    font_texture: gl::Texture,
}

impl Graphics {
    pub fn new() -> Self {
        let program = graphics::catalog::make_program(
            include_bytes!("damage_indicator.vert"),
            include_bytes!("damage_indicator.frag"),
        );

        let vertex_positions = Self::make_vertex_positions();
        let vertex_count = vertex_positions.len();

        let vertex_position_buffer = gl::Buffer::new();
        gl::named_buffer_data(&vertex_position_buffer, &vertex_positions,
                              gl::DataStoreUsage::StaticDraw);

        let vertex_texcoord_buffer = gl::Buffer::new();

        let positions_buffer = gl::Buffer::new();

        let ages_buffer = gl::Buffer::new();

        let vertex_array = gl::VertexArray::new();

        gl::bind_vertex_array(&vertex_array);

        gl::enable_vertex_attrib_array(0);
        gl::bind_buffer(gl::BufferTarget::ArrayBuffer,
                        &vertex_position_buffer);
        gl::vertex_attrib_pointer::<Vector2<f32>>(0, false);

        gl::enable_vertex_attrib_array(1);
        gl::bind_buffer(gl::BufferTarget::ArrayBuffer,
                        &vertex_texcoord_buffer);
        gl::vertex_attrib_pointer::<Vector2<f32>>(1, false);

        gl::enable_vertex_attrib_array(2);
        gl::bind_buffer(gl::BufferTarget::ArrayBuffer, &positions_buffer);
        gl::vertex_attrib_pointer::<Vector2<f32>>(2, false);
        gl::vertex_attrib_divisor(2, 1);

        gl::enable_vertex_attrib_array(3);
        gl::bind_buffer(gl::BufferTarget::ArrayBuffer, &ages_buffer);
        gl::vertex_attrib_pointer::<f32>(3, false);
        gl::vertex_attrib_divisor(3, 1);

        let font_texture = Self::make_font_texture();

        Graphics{program, vertex_array, vertex_count,
                 _vertex_position_buffer: vertex_position_buffer,
                 vertex_texcoord_buffer, positions_buffer, ages_buffer,
                 font_texture}
    }

    fn make_vertex_positions() -> [Vector2<f32>; 6] {
        [Vector2::new( 0.2,  0.2),
         Vector2::new(-0.2,  0.2),
         Vector2::new(-0.2, -0.2),

         Vector2::new(-0.2, -0.2),
         Vector2::new( 0.2, -0.2),
         Vector2::new( 0.2,  0.2)]
    }

    fn make_font_texture() -> gl::Texture {
        let (width, height, data) = {
            let decoder = png::Decoder::new(&include_bytes!(concat!(env!("OUT_DIR"), "/world/effect/damage_indicator/graphics/font.png"))[..]);
            let (info, mut reader) = decoder.read_info().unwrap();
            let mut data = vec![0; info.buffer_size()];
            reader.next_frame(&mut data).unwrap();
            (info.width as usize, info.height as usize, data)
        };

        let texture = gl::Texture::new(gl::TextureTarget::Texture2D);
        gl::bind_texture(gl::TextureTarget::Texture2D, &texture);
        unsafe {
            raw_gl::TextureParameteri(texture.handle(),
                                      raw_gl::TEXTURE_MIN_FILTER,
                                      raw_gl::LINEAR as raw_gl::types::GLint);
            raw_gl::TextureParameteri(texture.handle(),
                                      raw_gl::TEXTURE_MAG_FILTER,
                                      raw_gl::LINEAR as raw_gl::types::GLint);
        }
        unsafe {
            gl::tex_image_2d(gl::TextureTarget::Texture2D, 0,
                             gl::TextureInternalFormat::RGBA, width, height,
                             gl::TextureFormat::RGBA,
                             gl::TextureDataType::UnsignedByte,
                             &data);
        }
        gl::generate_mipmap(gl::TextureTarget::Texture2D);
        texture
    }

    pub fn draw(&self, damage_indicators: &DamageIndicatorSet,
                pmat: Matrix4<f32>, vmat: Matrix4<f32>, mmat: Matrix4<f32>) {
        gl::bind_vertex_array(&self.vertex_array);

        gl::draw_buffer(gl::ColorBuffer::Back);

        // FIXME(rightfold): Recompute these only when the damage indicators
        //                   change, instead of on every frame.
        let mut texcoords = Vec::with_capacity(damage_indicators.size());
        for _ in 0 .. damage_indicators.size() {
            let (w, h) = (6.0 / 128.0, 9.0 / 128.0);

            texcoords.extend(&[
                Vector2::new(  w, 0.0),
                Vector2::new(0.0, 0.0),
                Vector2::new(0.0,   h),

                Vector2::new(0.0,   h),
                Vector2::new(  w,   h),
                Vector2::new(  w, 0.0),
            ]);
        };
        gl::named_buffer_data(&self.vertex_texcoord_buffer, &texcoords,
                              gl::DataStoreUsage::DynamicDraw);

        gl::use_program(&self.program);

        let texture_unit = gl::TextureUnit(0);
        gl::active_texture(texture_unit);
        gl::bind_texture(gl::TextureTarget::Texture2D, &self.font_texture);

        gl::uniform(0, pmat * vmat * mmat);
        gl::uniform(1, texture_unit);

        gl::named_buffer_data(&self.positions_buffer,
                              soa_array!(damage_indicators, positions),
                              gl::DataStoreUsage::DynamicDraw);

        gl::named_buffer_data(&self.ages_buffer,
                              soa_array!(damage_indicators, ages),
                              gl::DataStoreUsage::StreamDraw);

        gl::draw_arrays_instanced(gl::PrimitiveType::Triangles,
                                  self.vertex_count,
                                  damage_indicators.size());
    }
}
