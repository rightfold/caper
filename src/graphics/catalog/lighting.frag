void lighting(in float brightness, in vec4 color, out vec4 target) {
  vec4 ambient = vec4(0.2, 0.2, 0.2, 1.0);
  target = color * ambient + vec4(brightness * color.rgb, color.a);
}
