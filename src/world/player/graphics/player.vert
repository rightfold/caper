#define LIGHT_HEIGHT 1.5
#define LIGHT_INTENSITY 2.0

layout(location = 0) in vec3 vertex_position;
layout(location = 1) in vec3 vertex_normal;

layout(location = 0) uniform mat4 pmat;
layout(location = 1) uniform mat4 vmat;
layout(location = 2) uniform mat4 mmat;
layout(location = 3) uniform vec2 light_position;

out float brightness;

void main() {
  mat4 vmmat = vmat * mmat;

  gl_Position = pmat * vmmat * vec4(vertex_position, 1.0);

  diffuse_lighting(vec3(vmmat * vec4(vertex_position, 1.0)),
                   vec3(vmmat * vec4(vertex_normal, 0.0)),
                   vec3(vmat * vec4(light_position, LIGHT_HEIGHT, 1.0)),
                   LIGHT_INTENSITY, brightness);
}
