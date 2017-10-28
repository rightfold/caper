#define LIGHT_HEIGHT 1.5
#define LIGHT_INTENSITY 2.0

#define SECTOR_SIZE 32

layout(location = 0) in vec2 vertex_position;
layout(location = 1) in uint tile_material;

layout(location = 0) uniform mat4 pmat;
layout(location = 1) uniform mat4 vmat;
layout(location = 2) uniform mat4 mmat;
layout(location = 3) uniform vec2 light_position;
layout(location = 4) uniform ivec2 sector_id;

flat out uint material;
out float brightness;

void main() {
  int col = sector_id.x * SECTOR_SIZE + gl_InstanceID % SECTOR_SIZE;
  int row = sector_id.y * SECTOR_SIZE + gl_InstanceID / SECTOR_SIZE;
  vec2 tile = vec2(sqrt(3.0) * (float(col) + 0.5 * float(row & 1)),
                   3.0 / 2.0 * float(row));

  mat4 mmmat = mat4(vec4(1.0, 0.0, 0.0, 0.0),
                    vec4(0.0, 1.0, 0.0, 0.0),
                    vec4(0.0, 0.0, 1.0, 0.0),
                    vec4(tile,     0.0, 1.0));

  mat4 vmmmat = vmat * mmat * mmmat;

  gl_Position = pmat * vmmmat * vec4(vertex_position, 0.0, 1.0);

  vec3 vertex_normal = vec3(0.0, 0.0, 1.0);

  material = tile_material;

  diffuse_lighting(vec3(vmmmat * vec4(vertex_position, 0.0, 1.0)),
                   vec3(vmmmat * vec4(vertex_normal, 0.0)),
                   vec3(vmat * vec4(light_position, LIGHT_HEIGHT, 1.0)),
                   LIGHT_INTENSITY, brightness);
}
