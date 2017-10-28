#define LIGHT_HEIGHT 1.5
#define LIGHT_INTENSITY 10.0

layout(location = 0) in vec3 vertex_position;
layout(location = 1) in vec3 vertex_normal;

layout(location = 0) uniform mat4 world_transform;
layout(location = 1) uniform mat4 model_transform;
layout(location = 2) uniform vec2 light_position;

out float brightness;

void main() {
  gl_Position = world_transform * model_transform * vec4(vertex_position, 1.0);

  diffuse_lighting(vec3(light_position, LIGHT_HEIGHT), LIGHT_INTENSITY,
                   world_transform, model_transform,
                   gl_Position.xyz, vertex_normal,
                   brightness);
}
