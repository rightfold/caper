use std::marker::PhantomData;
use std::mem;
use std::os::raw::c_void;

use cgmath;
use gl;



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArrayComponentDataType {
    UnsignedShort = gl::UNSIGNED_SHORT as isize,
    Float = gl::FLOAT as isize,
}



#[derive(Debug)]
pub struct Buffer<T>(gl::types::GLuint, PhantomData<T>);

impl<T> Buffer<T> {
    pub fn new() -> Self {
        unsafe {
            let mut handle = mem::uninitialized();
            gl::CreateBuffers(1, &mut handle);
            Buffer(handle, PhantomData)
        }
    }

    pub fn handle(&self) -> gl::types::GLuint {
        self.0
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.0);
        }
    }
}

impl<T> !Sync for Buffer<T> { }
impl<T> !Send for Buffer<T> { }



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BufferTarget {
    ArrayBuffer = gl::ARRAY_BUFFER as isize,
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER as isize,
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorBuffer {
    Back = gl::BACK as isize,
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataStoreUsage {
    StaticDraw  = gl::STATIC_DRAW  as isize,
    StreamDraw  = gl::STREAM_DRAW  as isize,
    DynamicDraw = gl::DYNAMIC_DRAW as isize,
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndexDataType {
    UnsignedInt = gl::UNSIGNED_INT as isize,
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveType {
    Triangles = gl::TRIANGLES as isize,
    TriangleFan = gl::TRIANGLE_FAN as isize,
}



#[derive(Debug)]
pub struct Program(gl::types::GLuint);

impl Program {
    pub fn new() -> Self {
        unsafe {
            let handle = gl::CreateProgram();
            Program(handle)
        }
    }

    pub fn handle(&self) -> gl::types::GLuint {
        self.0
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }
}

impl !Sync for Program { }
impl !Send for Program { }



#[derive(Debug)]
pub struct Shader(gl::types::GLuint);

impl Shader {
    pub fn new(shader_type: ShaderType) -> Self {
        unsafe {
            let handle = gl::CreateShader(shader_type as gl::types::GLenum);
            Shader(handle)
        }
    }

    pub fn handle(&self) -> gl::types::GLuint {
        self.0
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.0);
        }
    }
}

impl !Sync for Shader { }
impl !Send for Shader { }



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER as isize,
    FragmentShader = gl::FRAGMENT_SHADER as isize,
}



#[derive(Debug)]
pub struct Texture(gl::types::GLuint);

impl Texture {
    pub fn new(target: TextureTarget) -> Self {
        unsafe {
            let mut handle = mem::uninitialized();
            gl::CreateTextures(target as gl::types::GLenum, 1, &mut handle);
            Texture(handle)
        }
    }

    pub fn handle(&self) -> gl::types::GLuint {
        self.0
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.0);
        }
    }
}

impl !Sync for Texture { }
impl !Send for Texture { }



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureDataType {
    UnsignedByte = gl::UNSIGNED_BYTE as isize,
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureInternalFormat {
    RGBA = gl::RGBA as isize,
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureFormat {
    RGBA = gl::RGBA as isize,
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureTarget {
    Texture2D = gl::TEXTURE_2D as isize,
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextureUnit(pub usize);



#[derive(Debug)]
pub struct VertexArray(gl::types::GLuint);

impl VertexArray {
    pub fn new() -> Self {
        unsafe {
            let mut handle = mem::uninitialized();
            gl::CreateVertexArrays(1, &mut handle);
            VertexArray(handle)
        }
    }

    pub fn handle(&self) -> gl::types::GLuint {
        self.0
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.0);
        }
    }
}

impl !Sync for VertexArray { }
impl !Send for VertexArray { }



pub enum ArrayComponentDataTargetFloat { }
pub enum ArrayComponentDataTargetInteger { }

pub unsafe trait ArrayComponentData {
    type Target;
    fn component_count() -> usize;
    fn component_data_type() -> ArrayComponentDataType;
}

unsafe impl ArrayComponentData for f32 {
    type Target = ArrayComponentDataTargetFloat;

    fn component_count() -> usize {
        1
    }

    fn component_data_type() -> ArrayComponentDataType {
        ArrayComponentDataType::Float
    }
}

unsafe impl ArrayComponentData for cgmath::Rad<f32> {
    type Target = ArrayComponentDataTargetFloat;

    fn component_count() -> usize {
        1
    }

    fn component_data_type() -> ArrayComponentDataType {
        ArrayComponentDataType::Float
    }
}

unsafe impl ArrayComponentData for cgmath::Vector2<f32> {
    type Target = ArrayComponentDataTargetFloat;

    fn component_count() -> usize {
        2
    }

    fn component_data_type() -> ArrayComponentDataType {
        ArrayComponentDataType::Float
    }
}

unsafe impl ArrayComponentData for cgmath::Vector3<f32> {
    type Target = ArrayComponentDataTargetFloat;

    fn component_count() -> usize {
        3
    }

    fn component_data_type() -> ArrayComponentDataType {
        ArrayComponentDataType::Float
    }
}



pub unsafe trait BufferData { }

unsafe impl BufferData for f32 { }
unsafe impl BufferData for u32 { }

unsafe impl BufferData for cgmath::Rad<f32> { }
unsafe impl BufferData for cgmath::Vector2<f32> { }
unsafe impl BufferData for cgmath::Vector3<f32> { }



pub unsafe trait IndexData {
    fn index_data_type() -> IndexDataType;
}

unsafe impl IndexData for u32 {
    fn index_data_type() -> IndexDataType {
        IndexDataType::UnsignedInt
    }
}



pub trait Uniform {
    fn uniform(self, location: usize);
}

impl Uniform for TextureUnit {
    fn uniform(self, location: usize) {
        unsafe {
            gl::Uniform1i(location as gl::types::GLint, self.0 as i32);
        }
    }
}

impl Uniform for cgmath::Vector2<i32> {
    fn uniform(self, location: usize) {
        unsafe {
            gl::Uniform2i(location as gl::types::GLint, self.x, self.y);
        }
    }
}

impl Uniform for cgmath::Vector2<f32> {
    fn uniform(self, location: usize) {
        unsafe {
            gl::Uniform2f(location as gl::types::GLint, self.x, self.y);
        }
    }
}

impl Uniform for cgmath::Matrix4<f32> {
    fn uniform(self, location: usize) {
        unsafe {
            let value: &[f32; 16] = self.as_ref();
            gl::UniformMatrix4fv(location as gl::types::GLint,
                                 1 as gl::types::GLsizei,
                                 gl::FALSE,
                                 value.as_ptr());
        }
    }
}



pub fn active_texture(texture: TextureUnit) {
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0 + texture.0 as gl::types::GLenum);
    }
}

pub fn attach_shader(program: &Program, shader: &Shader) {
    unsafe {
        gl::AttachShader(program.handle(), shader.handle());
    }
}

pub fn bind_buffer<T>(target: BufferTarget, buffer: &Buffer<T>) {
    unsafe {
        gl::BindBuffer(target as gl::types::GLenum, buffer.handle());
    }
}

pub fn bind_texture(target: TextureTarget, texture: &Texture) {
    unsafe {
        gl::BindTexture(target as gl::types::GLenum, texture.handle());
    }
}

pub fn bind_vertex_array(vertex_array: &VertexArray) {
    unsafe {
        gl::BindVertexArray(vertex_array.handle());
    }
}

pub fn compile_shader(shader: &Shader) {
    unsafe {
        gl::CompileShader(shader.handle());
    }
}

pub fn draw_arrays_instanced(mode: PrimitiveType, index_count: usize,
                             primitive_count: usize) {
    unsafe {
        gl::DrawArraysInstanced(mode as gl::types::GLenum,
                                0 as gl::types::GLint,
                                index_count as gl::types::GLsizei,
                                primitive_count as gl::types::GLsizei);
    }
}

pub fn draw_buffer(color_buffer: ColorBuffer) {
    unsafe {
        gl::DrawBuffer(color_buffer as gl::types::GLenum);
    }
}

pub fn draw_elements<I>(mode: PrimitiveType, index_count: usize)
    where I: IndexData {
    unsafe {
        gl::DrawElements(mode as gl::types::GLenum,
                         index_count as gl::types::GLsizei,
                         I::index_data_type() as gl::types::GLenum,
                         0 as *const c_void);
    }
}

pub fn draw_elements_instanced<I>(mode: PrimitiveType, index_count: usize,
                                  primitive_count: usize)
    where I: IndexData {
    unsafe {
        gl::DrawElementsInstanced(mode as gl::types::GLenum,
                                  index_count as gl::types::GLsizei,
                                  I::index_data_type() as gl::types::GLenum,
                                  0 as *const c_void,
                                  primitive_count as gl::types::GLsizei);
    }
}

pub fn enable_vertex_attrib_array(index: usize) {
    unsafe {
        gl::EnableVertexAttribArray(index as gl::types::GLuint);
    }
}

pub fn generate_mipmap(target: TextureTarget) {
    unsafe {
        gl::GenerateMipmap(target as gl::types::GLenum);
    }
}

pub fn link_program(program: &Program) {
    unsafe {
        gl::LinkProgram(program.handle());
    }
}

pub fn named_buffer_data<T>(buffer: &Buffer<T>, data: &[T], usage: DataStoreUsage)
    where T: BufferData {
    unsafe {
        let data_size = (mem::size_of::<T>() * data.len()) as isize;
        let data_ptr = data.as_ptr() as *const c_void;
        gl::NamedBufferData(buffer.handle(),
                            data_size, data_ptr,
                            usage as gl::types::GLenum);
    }
}

pub fn shader_source<I>(shader: &Shader, sources: I)
    where I: IntoIterator, I::Item: AsRef<[u8]> {
    let mut strings = Vec::<*const gl::types::GLchar>::new();
    let mut lengths = Vec::<gl::types::GLint>::new();
    for source in sources {
        strings.push(source.as_ref().as_ptr() as *const gl::types::GLchar);
        lengths.push(source.as_ref().len() as gl::types::GLint);
    }
    unsafe {
        gl::ShaderSource(shader.handle(), strings.len() as gl::types::GLsizei,
                         strings.as_ptr(), lengths.as_ptr());
    }
}

pub unsafe fn tex_image_2d(target: TextureTarget, level: usize,
                           internal_format: TextureInternalFormat, width: usize,
                           height: usize, format: TextureFormat,
                           data_type: TextureDataType, data: &[u8]) {
    gl::TexImage2D(target as gl::types::GLenum,
                   level as gl::types::GLint,
                   internal_format as gl::types::GLint,
                   width as gl::types::GLsizei,
                   height as gl::types::GLsizei,
                   0,
                   format as gl::types::GLenum,
                   data_type as gl::types::GLenum,
                   data.as_ptr() as *const c_void);
}

pub fn uniform<T>(location: usize, value: T)
    where T: Uniform {
    value.uniform(location);
}

pub fn use_program(program: &Program) {
    unsafe {
        gl::UseProgram(program.handle());
    }
}

pub fn vertex_attrib_divisor(index: usize, divisor: usize) {
    unsafe {
        gl::VertexAttribDivisor(index as gl::types::GLuint,
                                divisor as gl::types::GLuint);
    }
}

pub fn vertex_attrib_pointer<T>(index: usize, normalized: bool)
    where T: ArrayComponentData<Target=ArrayComponentDataTargetFloat> {
    unsafe {
        gl::VertexAttribPointer(index as gl::types::GLuint,
                                T::component_count() as gl::types::GLint,
                                T::component_data_type() as gl::types::GLenum,
                                normalized as gl::types::GLboolean,
                                0,
                                0 as *const c_void);
    }
}

pub fn vertex_attrib_i_pointer<T>(index: usize)
    where T: ArrayComponentData<Target=ArrayComponentDataTargetInteger> {
    unsafe {
        gl::VertexAttribIPointer(index as gl::types::GLuint,
                                 T::component_count() as gl::types::GLint,
                                 T::component_data_type() as gl::types::GLenum,
                                 0,
                                 0 as *const c_void);
    }
}
