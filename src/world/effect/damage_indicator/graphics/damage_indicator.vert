#define BASE_HEIGHT 0.5

layout(location = 0) in vec2 vertex_position;
layout(location = 1) in vec2 vertex_texcoord;
layout(location = 2) in vec2 position;
layout(location = 3) in float age;
layout(location = 4) in int value;

layout(location = 0) uniform mat4 pvmmat;

out vec2 texcoord;

void main() {
  mat4 mmmat = mat4(vec4(       1.0,        0.0,               0.0, 0.0),
                    vec4(       0.0,        1.0,               0.0, 0.0),
                    vec4(       0.0,        0.0,               1.0, 0.0),
                    vec4(position.x, position.y, BASE_HEIGHT + age, 1.0));
  gl_Position = pvmmat * mmmat * vec4(vertex_position, 0.0, 1.0);

  texcoord = vertex_texcoord + vec2(float(value) * 8.0 / 128.0, 0.0);
}
