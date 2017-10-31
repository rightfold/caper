#[doc(hidden)]
pub trait MakeId {
    fn make_id(id: usize) -> Self;
}

#[doc(hidden)]
pub fn make_id<T: MakeId>(id: usize) -> T {
    T::make_id(id)
}

#[macro_export]
macro_rules! soa_spawn {
    { $self:expr, $($name:ident : $value:expr,)* } => {{
        let this = $self;

        let id = $crate::macros::make_id(this.next_id);
        this.next_id += 1;

        let index = this.ids_by_index.len();
        this.indices_by_id.insert(id, index);
        this.ids_by_index.push(id);

        $(this.arrays.$name.push($value);)*

        id
    }};
}

#[macro_export]
macro_rules! soa {
    {
        $name:ident,

        $id:ident,

        $(array $array_name:ident : $array_type:ty,)*

        $(scalar $scalar_name:ident : $scalar_type:ty = $scalar_initial:expr,)*
    } => {
        #[derive(Debug)]
        pub struct $name {
            next_id: usize,
            indices_by_id: $crate::std::collections::HashMap<$id, usize>,
            ids_by_index: Vec<$id>,
            #[doc(hidden)] pub arrays: arrays::Arrays,
            pub scalars: scalars::Scalars,
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $id(usize);

        impl $crate::macros::MakeId for $id {
            fn make_id(id: usize) -> Self {
                $id(id)
            }
        }

        impl $name {
            pub fn new() -> Self {
                $name{
                    next_id: 0,
                    indices_by_id: $crate::std::collections::HashMap::new(),
                    ids_by_index: Vec::new(),
                    arrays: arrays::Arrays::new(),
                    scalars: scalars::Scalars::new(),
                }
            }

            pub fn size(&self) -> usize {
                self.ids_by_index.len()
            }

            pub fn ids(&self) -> &[$id] {
                &self.ids_by_index
            }

            pub fn despawn(&mut self, id: $id) {
                let &index = self.indices_by_id.get(&id).unwrap();
                let &last_id = self.ids_by_index.last().unwrap();
                self.ids_by_index.swap_remove(index);
                self.indices_by_id.insert(last_id, index);
                $(self.arrays.$array_name.swap_remove(index);)*
            }
        }

        mod arrays {
            #[allow(unused_imports)]
            use super::*;

            #[derive(Debug)]
            pub struct Arrays {
                $(pub $array_name: Vec<$array_type>,)*
            }

            impl Arrays {
                pub fn new() -> Self {
                    Arrays{$($array_name: Vec::new(),)*}
                }
            }
        }

        pub mod scalars {
            #[allow(unused_imports)]
            use super::*;

            #[derive(Debug)]
            pub struct Scalars {
                $(pub $scalar_name: $scalar_type,)*
            }

            impl Scalars {
                pub fn new() -> Self {
                    Scalars{$($scalar_name: $scalar_initial,)*}
                }
            }
        }
    };
}

#[macro_export]
macro_rules! monster_set {
    {
        $name:ident,

        $id:ident,

        $(attribute $attribute_name:ident : $attribute_type:ty,)*

        $(visible(location = $visible_location:expr) $visible_name:ident : $visible_type:ty,)*

        $(state $state_name:ident : $state_type:ty = $state_initial:expr,)*

        spawn($spawn_rng:ident, $spawn_position:ident) {
            $($spawn_attribute_name:ident : $spawn_attribute_value:expr,)*
        },

        graphics {
            model : $model:expr,

            vertex_shader   : $vertex_shader:expr,
            fragment_shader : $fragment_shader:expr,
        },
    } => {
        soa! {
            $name,

            $id,

            $(array $attribute_name: $attribute_type,)*
            $(array $visible_name: $visible_type,)*

            $(scalar $state_name: $state_type = $state_initial,)*
            scalar graphics: graphics::Graphics = graphics::Graphics::new(),
        }

        impl $crate::world::monster::MonsterSet for $name {
            type Id = $id;

            fn ids(&self) -> &[$id] {
                self.ids()
            }

            fn positions(&self) -> &[$crate::cgmath::Vector2<f32>] {
                &self.arrays.positions
            }

            fn healths(&self) -> &[f32] {
                &self.arrays.healths
            }

            fn spawn<R>(&mut self, $spawn_rng: &mut R,
                        $spawn_position: $crate::cgmath::Vector2<f32>) -> $id
                where R: $crate::rand::Rng {
                soa_spawn! {
                    self,
                    $($spawn_attribute_name : $spawn_attribute_value,)*
                }
            }

            fn despawn(&mut self, id: $id) {
                self.despawn(id);
            }

            fn draw(&self, pmat: $crate::cgmath::Matrix4<f32>,
                    vmat: $crate::cgmath::Matrix4<f32>,
                    mmat: $crate::cgmath::Matrix4<f32>,
                    light_position: $crate::cgmath::Vector2<f32>) {
                self.scalars.graphics.draw(self, pmat, vmat, mmat, light_position);
            }
        }

        mod graphics {
            #[allow(unused_imports)]
            use super::*;

            use $crate::graphics::gl;

            #[derive(Debug)]
            pub struct Graphics {
                program: gl::Program,

                vertex_array: gl::VertexArray,

                _vertex_position_buffer: gl::Buffer<$crate::cgmath::Vector3<f32>>,
                _vertex_normal_buffer: gl::Buffer<$crate::cgmath::Vector3<f32>>,

                vertex_index_count: usize,
                vertex_index_buffer: gl::Buffer<u32>,

                $(pub $visible_name: gl::Buffer<$visible_type>,)*
            }

            impl Graphics {
                pub fn new() -> Self {
                    let program = Self::make_program();

                    let model = $model;

                    let vertex_position_buffer = gl::Buffer::new();
                    gl::named_buffer_data(&vertex_position_buffer,
                                          &model.vertex_positions,
                                          gl::DataStoreUsage::StaticDraw);

                    let vertex_normal_buffer = gl::Buffer::new();
                    gl::named_buffer_data(&vertex_normal_buffer,
                                          &model.vertex_normals,
                                          gl::DataStoreUsage::StaticDraw);

                    let vertex_index_count = model.vertex_indices.len();
                    let vertex_index_buffer = gl::Buffer::new();
                    gl::named_buffer_data(&vertex_index_buffer,
                                          &model.vertex_indices,
                                          gl::DataStoreUsage::StaticDraw);

                    $(let $visible_name = gl::Buffer::new();)*

                    let vertex_array = Self::make_vertex_array(
                        &vertex_position_buffer,
                        &vertex_normal_buffer,
                        $(&$visible_name,)*
                    );

                    Graphics{
                        program,

                        vertex_array,

                        _vertex_position_buffer: vertex_position_buffer,
                        _vertex_normal_buffer: vertex_normal_buffer,

                        vertex_index_count,
                        vertex_index_buffer,

                        $($visible_name,)*
                    }
                }

                fn make_program() -> gl::Program {
                    $crate::graphics::catalog::make_program(
                        $vertex_shader,
                        $fragment_shader,
                    )
                }

                fn make_vertex_array(vertex_position_buffer: &gl::Buffer<$crate::cgmath::Vector3<f32>>,
                                     vertex_normal_buffer: &gl::Buffer<$crate::cgmath::Vector3<f32>>,
                                     $($visible_name: &gl::Buffer<$visible_type>,)*) -> gl::VertexArray {
                    let vertex_array = gl::VertexArray::new();

                    gl::bind_vertex_array(&vertex_array);

                    gl::enable_vertex_attrib_array(0);
                    gl::bind_buffer(gl::BufferTarget::ArrayBuffer, vertex_position_buffer);
                    gl::vertex_attrib_pointer::<$crate::cgmath::Vector3<f32>>(0, false);

                    gl::enable_vertex_attrib_array(1);
                    gl::bind_buffer(gl::BufferTarget::ArrayBuffer, vertex_normal_buffer);
                    gl::vertex_attrib_pointer::<$crate::cgmath::Vector3<f32>>(1, false);

                    $({
                        gl::enable_vertex_attrib_array($visible_location);
                        gl::bind_buffer(gl::BufferTarget::ArrayBuffer, $visible_name);
                        gl::vertex_attrib_pointer::<$visible_type>($visible_location, false);
                        gl::vertex_attrib_divisor($visible_location, 1);
                    })*

                    vertex_array
                }

                pub fn draw(&self, monster_set: &$name,
                            pmat: $crate::cgmath::Matrix4<f32>,
                            vmat: $crate::cgmath::Matrix4<f32>,
                            mmat: $crate::cgmath::Matrix4<f32>,
                            light_position: $crate::cgmath::Vector2<f32>) {
                    gl::bind_vertex_array(&self.vertex_array);

                    $({
                        gl::named_buffer_data(&self.$visible_name,
                                              soa_array!(monster_set, $visible_name),
                                              gl::DataStoreUsage::StreamDraw);
                    })*

                    gl::bind_buffer(gl::BufferTarget::ElementArrayBuffer,
                                    &self.vertex_index_buffer);

                    gl::draw_buffer(gl::ColorBuffer::Back);

                    gl::use_program(&self.program);

                    gl::uniform(0, pmat);
                    gl::uniform(1, vmat);
                    gl::uniform(2, mmat);
                    gl::uniform(3, light_position);

                    gl::draw_elements_instanced::<u32>(gl::PrimitiveType::Triangles,
                                                       self.vertex_index_count,
                                                       monster_set.ids().len());
                }
            }
        }
    };
}

#[macro_export]
macro_rules! soa_array {
    ($self:expr, $field:ident) => {
        &$self.arrays.$field[..]
    };
    ($self:expr, mut $field:ident) => {
        &mut $self.arrays.$field[..]
    };
}
