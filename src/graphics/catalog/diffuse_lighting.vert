void diffuse_lighting(in vec3 light_position, in float light_intensity,
                      in mat4 world_transform, in mat4 model_transform,
                      in vec3 world_vertex_position, in vec3 vertex_normal,
                      out float brightness) {
  vec3 world_model_normal = (world_transform * model_transform * vec4(vertex_normal, 0.0)).xyz;
  vec3 world_light = (world_transform * vec4(light_position, 1.0)).xyz - world_vertex_position;
  float light_distance = length(world_light);
  brightness =
    light_intensity
    * clamp(dot(normalize(world_light), world_model_normal), 0, 1)
    / pow(light_distance, 2);
}
