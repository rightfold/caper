#version 150 core

in vec3 position;

uniform Transform {
  mat4 world_transform;
  mat4 model_transform;
};

out vec3 interpolated_color;

void main() {
  gl_Position = world_transform * model_transform * vec4(position, 1.0);
  interpolated_color = position / 2.0;
}
