in vec2 texcoord;

layout(location = 1) uniform sampler2D tex;

layout(location = 0) out vec4 target;

void main() {
  target = texture(tex, texcoord);
}
