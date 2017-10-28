void diffuse_lighting(in vec3 vm_vertex_position, in vec3 vm_vertex_normal,
                      in vec3 light_position, in float light_intensity,
                      out float brightness) {
  float light_distance = length(light_position - vm_vertex_position);
  vec3 light = normalize(light_position - vm_vertex_position);
  brightness =
    light_intensity
    * clamp(dot(vm_vertex_normal, light), 0.0, 1.0)
    / (1.0 + (0.25 * light_distance * light_distance));
}
