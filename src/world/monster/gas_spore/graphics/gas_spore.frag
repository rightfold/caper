in float brightness;

layout(location = 0) out vec4 target;

void main() {
  vec4 color = vec4(0.23, 0.18, 0.06, 1.0);
  lighting(brightness, color, target);
}
