#define LIGHT_HEIGHT 1.5
#define LIGHT_INTENSITY 2.0

layout(location = 0) in vec3 vertex_position;
layout(location = 1) in vec3 vertex_normal;
layout(location = 2) in vec2 model_position;
layout(location = 3) in float model_angle;
layout(location = 4) in float model_altitude;

layout(location = 0) uniform mat4 pmat;
layout(location = 1) uniform mat4 vmat;
layout(location = 2) uniform mat4 mmat;
layout(location = 3) uniform vec2 light_position;

out float brightness;

void main() {
  mat4 mmmat = mat4(vec4( cos(model_angle), sin(model_angle),            0.0, 0.0),
                    vec4(-sin(model_angle), cos(model_angle),            0.0, 0.0),
                    vec4(              0.0,              0.0,            1.0, 0.0),
                    vec4( model_position.x, model_position.y, model_altitude, 1.0));
  mat4 vmmmat = vmat * mmat * mmmat;

  gl_Position = pmat * vmmmat * vec4(vertex_position, 1.0);

  diffuse_lighting(vec3(vmmmat * vec4(vertex_position, 1.0)),
                   vec3(vmmmat * vec4(vertex_normal,   0.0)),
                   vec3(vmat * vec4(light_position, LIGHT_HEIGHT, 1.0)),
                   LIGHT_INTENSITY, brightness);
}
