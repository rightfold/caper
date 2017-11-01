in float brightness;

layout(location = 0) out vec4 target;

void main() {
  vec4 color = vec4(0.22, 0.21, 0.23, 1.0);
  lighting(brightness, color, target);
}
