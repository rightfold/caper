#version 330 core
#extension GL_ARB_explicit_uniform_location : require

layout(location = 0) in vec3 vertex;
layout(location = 1) in vec2 position;
layout(location = 2) in float angle;

layout(location = 0) uniform mat4 world_transform;

void main() {
  mat4 model_transform = mat4(vec4( cos(angle), sin(angle), 0, 0),
                              vec4(-sin(angle), cos(angle), 0, 0),
                              vec4(          0,          0, 1, 0),
                              vec4( position.x, position.y, 0, 1));
  gl_Position = world_transform * model_transform * vec4(vertex, 1.0);
}
