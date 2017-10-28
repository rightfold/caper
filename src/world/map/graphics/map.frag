#define MATERIAL_STONE 0x00u
#define MATERIAL_GRASS 0x01u

flat in uint material;
in float brightness;

layout(location = 0) out vec4 target;

void main() {
  vec4 color;
  switch (material) {
  case MATERIAL_STONE:
    color = vec4(0.7, 0.7, 0.7, 1.0);
    break;
  case MATERIAL_GRASS:
    color = vec4(0.0, 1.0, 0.0, 1.0);
    break;
  }
  lighting(brightness, color, target);
}
