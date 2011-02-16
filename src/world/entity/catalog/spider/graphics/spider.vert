#version 330 core
#extension GL_ARB_explicit_uniform_location : require

layout(location = 0) in vec3 vertex_position;
layout(location = 1) in vec2 model_position;
layout(location = 2) in float model_angle;

layout(location = 0) uniform mat4 world_transform;

void main() {
  mat4 model_transform = mat4(vec4( cos(model_angle), sin(model_angle), 0, 0),
                              vec4(-sin(model_angle), cos(model_angle), 0, 0),
                              vec4(                0,                0, 1, 0),
                              vec4( model_position.x, model_position.y, 0, 1));
  gl_Position = world_transform * model_transform * vec4(vertex_position, 1.0);
}
