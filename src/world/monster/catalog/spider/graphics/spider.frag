in float brightness;

layout(location = 0) out vec4 target;

void main() {
  vec4 color = vec4(1.0, 0.0, 1.0, 1.0);
  lighting(brightness, color, target);
}
