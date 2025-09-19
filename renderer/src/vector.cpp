#include "vector.h"
#include <cmath>

vec3_t vec3_rotate_x(vec3_t v, float angle) {
  vec3_t rotated_vector = {
    .x = v.x,
    .y = v.y * cos(angle) - v.z * sin(angle),
    .z = v.z * sin(angle) + v.z * cos(angle),
  };

  return rotated_vector;
}

vec3_t vec3_rotate_y(vec3_t v, float angle) {
  vec3_t rotated_vector = {
    .x = v.x * cos(angle) - v.z * sin(angle),
    .y = v.y,
    .z = v.z * sin(angle) + v.z * cos(angle),
  };

  return rotated_vector;
}

vec3_t vec3_rotate_z(vec3_t v, float angle) {
  vec3_t rotated_vector = {
    .x = v.x * cos(angle) - v.y * sin(angle),
    .y = v.y * sin(angle) - v.z * cos(angle),
    .z = v.z
  };

  return rotated_vector;
}
