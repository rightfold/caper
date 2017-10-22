#version 150 core

in vec2 position;

out vec3 interpolated_color;

uniform Transform {
  mat4 world_transform;
  mat4 model_transform;
};

void main() {
  gl_Position = world_transform * model_transform * vec4(position, 0.0, 1.0);
  interpolated_color = vec3(position, 0.5);
}
