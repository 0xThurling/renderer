#include "SDL_events.h"
#include "SDL_keycode.h"
#include "SDL_pixels.h"
#include "SDL_render.h"
#include "SDL_timer.h"
#include "array.h"
#include "display.h"
#include "mesh.h"
#include "triangle.h"
#include "vector.h"
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <iostream>

/////////////////////////////////////////////////////////////
//// Array of triangle that should be rendered frame by frame
/////////////////////////////////////////////////////////////
triangle_t* triangles_to_render = NULL;

/////////////////////////////////////////////////////////////
//// Array of triangle that should be rendered frame by frame
/////////////////////////////////////////////////////////////
bool is_running = false;
int previous_frame_time = 0;

vec3_t camera_position = {.x = 0, .y = 0, .z = -5};
float fov_factor = 640;

/////////////////////////////////////////////////////////////
//// Array of triangle that should be rendered frame by frame
/////////////////////////////////////////////////////////////
void setup() {
  color_buffer = (uint32_t*)malloc(sizeof(uint32_t) * window_width * window_height);

  color_buffer_texture = SDL_CreateTexture(
      renderer,
      SDL_PIXELFORMAT_ARGB8888,
      SDL_TEXTUREACCESS_STREAMING,
      window_width,
      window_height
  );
  
  load_cube_mesh_data();
}

/////////////////////////////////////////////////////////////
//// Poll system events and handle keyboard input
/////////////////////////////////////////////////////////////
void process_input() {
  SDL_Event event;
  SDL_PollEvent(&event);
  switch (event.type) {
    case SDL_QUIT:
      is_running = false;
      break;
    case SDL_KEYDOWN:
      if (event.key.keysym.sym == SDLK_ESCAPE) {
        is_running = false;
      }
      break;
  }
}

/////////////////////////////////////////////////////////////
//// Function that recieves a 3D vector and returns
//// a projected 2D point
/////////////////////////////////////////////////////////////
vec2_t project(vec3_t point) {
  vec2_t projected_point = {
    .x = (fov_factor * point.x) / point.z,
    .y = (fov_factor * point.y) / point.z
  };

  return projected_point;
}

// Function for othographic projections
vec2_t orthographic_project(vec3_t vector) {
  vec2_t projected_point = {
    .x = (fov_factor * vector.x),
    .y = (fov_factor * vector.y) 
  };

  return projected_point;
}

/////////////////////////////////////////////////////////////
//// Update function frame by frame with a fixed time step
/////////////////////////////////////////////////////////////
void update() {
  int time_to_wait = FRAME_TARGET_TIME - (SDL_GetTicks() - previous_frame_time);

  // Only delay execution if we are running too fast
  if (time_to_wait > 0 && time_to_wait <= FRAME_TARGET_TIME) {
    SDL_Delay(time_to_wait);
  }

  previous_frame_time = SDL_GetTicks();

  // Initialize the array of triangle to render
  triangles_to_render = NULL;

  mesh.rotation.x += 0.01;
  mesh.rotation.y += 0.01;
  mesh.rotation.z += 0.01;

  // Loop all triangle faces of our mesh
  int num_faces = array_length(mesh.faces);

  for (int i = 1; i < num_faces; i++) {
    face_t mesh_face = mesh.faces[i];

    vec3_t face_vertices[3];
    face_vertices[0] = mesh.vertices[mesh_face.a - 1];
    face_vertices[1] = mesh.vertices[mesh_face.b - 1];
    face_vertices[2] = mesh.vertices[mesh_face.c - 1];

    triangle_t projected_triangle;
  }
}







int main() {
    std::cout << "Hello, C++ World!" << std::endl;
    return 0;
}
