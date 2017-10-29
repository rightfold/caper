use graphics::gl;
use world::map;

pub fn make_program(vertex_shader_source: &[u8],
                    fragment_shader_source: &[u8]) -> gl::Program {
    let vert_shader = gl::Shader::new(gl::ShaderType::VertexShader);
    gl::shader_source(&vert_shader, &[
        &b"#version 330 core\n"[..],
        &b"#extension GL_ARB_explicit_uniform_location : require\n"[..],
        format!("#define CAPER_MAP_SECTOR_SIZE {}\n", map::SECTOR_SIZE).as_bytes(),
        vertex::DIFFUSE_LIGHTING,
        vertex_shader_source,
    ]);
    gl::compile_shader(&vert_shader);

    let frag_shader = gl::Shader::new(gl::ShaderType::FragmentShader);
    gl::shader_source(&frag_shader, &[
        &b"#version 330 core\n"[..],
        fragment::LIGHTING,
        fragment_shader_source,
    ]);
    gl::compile_shader(&frag_shader);

    let program = gl::Program::new();
    gl::attach_shader(&program, &vert_shader);
    gl::attach_shader(&program, &frag_shader);
    gl::link_program(&program);

    program
}

mod vertex {
    pub const DIFFUSE_LIGHTING: &'static [u8] = include_bytes!("diffuse_lighting.vert");
}

mod fragment {
    pub const LIGHTING: &'static [u8] = include_bytes!("lighting.frag");
}
