#version 330 core
#extension GL_ARB_explicit_uniform_location : require

#define LIGHT_HEIGHT 1.5
#define LIGHT_INTENSITY 10.0

layout(location = 0) in vec3 vertex_position;
layout(location = 1) in vec3 vertex_normal;
layout(location = 2) in vec2 model_position;
layout(location = 3) in float model_angle;

layout(location = 0) uniform mat4 world_transform;
layout(location = 1) uniform vec2 light_position;

out float brightness;

void main() {
  mat4 model_transform = mat4(vec4( cos(model_angle), sin(model_angle), 0, 0),
                              vec4(-sin(model_angle), cos(model_angle), 0, 0),
                              vec4(                0,                0, 1, 0),
                              vec4( model_position.x, model_position.y, 0, 1));
  mat4 world_model_transform = world_transform * model_transform;
  vec3 world_model_normal = (world_model_transform * vec4(vertex_normal, 0.0)).xyz;

  gl_Position = world_model_transform * vec4(vertex_position, 1.0);

  vec3 world_light = (world_transform * vec4(light_position, LIGHT_HEIGHT, 1.0)).xyz - gl_Position.xyz;
  float light_distance = length(world_light);
  brightness =
    LIGHT_INTENSITY
    * clamp(dot(normalize(world_light), world_model_normal), 0, 1)
    / pow(light_distance, 2);
}
