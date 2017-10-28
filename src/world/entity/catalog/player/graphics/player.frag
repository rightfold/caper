#version 330 core

in float brightness;

layout(location = 0) out vec4 target;

void main() {
  vec4 ambient = vec4(0.2, 0.2, 0.2, 1.0);
  vec4 color = vec4(1.0, 1.0, 1.0, 1.0);
  target = color * ambient + vec4(brightness * color.rgb, color.a);
}
