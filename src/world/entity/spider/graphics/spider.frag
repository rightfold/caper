#version 150 core

in vec3 interpolated_color;

out vec4 target;

void main () {
  target = vec4(interpolated_color, 1.0);
}
