#version 330 core
#extension GL_ARB_explicit_uniform_location : require

layout(location = 0) in vec3 vertex;

layout(location = 0) uniform mat4 world_model_transform;

void main() {
  gl_Position = world_model_transform * vec4(vertex, 1.0);
}
