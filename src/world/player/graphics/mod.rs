use cgmath::{Deg, Matrix4, Vector2, Vector3};

use graphics;
use graphics::gl;
use graphics::obj::Obj;
use world::item;
use world::player::*;

#[derive(Debug)]
pub struct Graphics {
    program: gl::Program,
    vertex_array: gl::VertexArray,
    _vertex_position_buffer: gl::Buffer<Vector3<f32>>,
    _vertex_normal_buffer: gl::Buffer<Vector3<f32>>,
    vertex_index_count: usize,
    vertex_index_buffer: gl::Buffer<u32>,

    equipment_carrying_mat: Matrix4<f32>,
    equipment_swinging_mat: Matrix4<f32>,

    sword: item::sword::graphics::Graphics,
}

impl Graphics {
    pub fn new() -> Self {
        let model: Obj<Vector3<f32>, Vector3<f32>> =
            Obj::read(include_str!(concat!(env!("OUT_DIR"),
                                           "/world/player/graphics/player.obj"))).unwrap();
        let equipment_carrying_mat = model.metas["Equipment (Carrying)"].transform;
        let equipment_swinging_mat = model.metas["Equipment (Swinging)"].transform;

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
        gl::bind_buffer(gl::BufferTarget::ArrayBuffer,
                        &vertex_position_buffer);
        gl::vertex_attrib_pointer::<Vector3<f32>>(0, false);

        gl::enable_vertex_attrib_array(1);
        gl::bind_buffer(gl::BufferTarget::ArrayBuffer,
                        &vertex_normal_buffer);
        gl::vertex_attrib_pointer::<Vector3<f32>>(1, false);

        let sword = item::sword::graphics::Graphics::new();

        Graphics{program, vertex_array,
                 _vertex_position_buffer: vertex_position_buffer,
                 _vertex_normal_buffer: vertex_normal_buffer,
                 vertex_index_count, vertex_index_buffer,

                 equipment_carrying_mat,
                 equipment_swinging_mat,

                 sword}
    }

    fn new_program() -> gl::Program {
        graphics::catalog::make_program(
            include_bytes!("player.vert"),
            include_bytes!("player.frag"),
        )
    }

    pub fn draw(&self, player: &Player, pmat: Matrix4<f32>, vmat: Matrix4<f32>,
                mut mmat: Matrix4<f32>, light_position: Vector2<f32>) {
        mmat = mmat
            * Matrix4::from_translation(player.position.extend(0.0))
            * Matrix4::from_angle_z(player.angle);

        gl::bind_vertex_array(&self.vertex_array);

        gl::bind_buffer(gl::BufferTarget::ElementArrayBuffer,
                        &self.vertex_index_buffer);

        gl::draw_buffer(gl::ColorBuffer::Back);

        gl::use_program(&self.program);

        gl::uniform(0, pmat);
        gl::uniform(1, vmat);
        gl::uniform(2, mmat);
        gl::uniform(3, light_position);

        self.draw_sword(pmat, vmat, mmat, light_position, &player.attack_state);
    }

    fn draw_sword(&self, pmat: Matrix4<f32>, vmat: Matrix4<f32>,
                  mut mmat: Matrix4<f32>, light_position: Vector2<f32>,
                  attack_state: &AttackState) {
        mmat = match attack_state {
            &AttackState::NotAttacking => mmat * self.equipment_carrying_mat,
            &AttackState::Attacking(progress) => mmat
                * self.equipment_swinging_mat
                * Matrix4::from_angle_z(Deg(progress * 110.0))
                * Matrix4::from_translation(Vector3::new(progress * 0.3, 0.0, 0.0)),
        };

        gl::draw_elements::<u32>(gl::PrimitiveType::Triangles,
                                 self.vertex_index_count);

        self.sword.draw(pmat, vmat, mmat, light_position);
    }
}
