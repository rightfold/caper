#version 330 core
#extension GL_ARB_explicit_uniform_location : require

#define SECTOR_SIZE 8

layout(location = 0) in vec2 vertex_position;

layout(location = 0) uniform mat4 world_transform;
layout(location = 1) uniform ivec2 sector_id;

void main() {
  int col = sector_id.x * SECTOR_SIZE + gl_InstanceID % SECTOR_SIZE;
  int row = sector_id.y * SECTOR_SIZE + gl_InstanceID / SECTOR_SIZE;

  float x = sqrt(3.0) * (float(col) + 0.5 * float(row & 1));
  float y = 3.0 / 2.0 * float(row);

  vec4 tile_position = vec4(x, y, 0.0, 1.0);

  gl_Position = world_transform * (tile_position + vec4(vertex_position, 0.0, 1.0));
}
