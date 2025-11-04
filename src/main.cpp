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
#include <ostream>
#include <string>

/////////////////////////////////////////////////////////////
//// Array of triangle that should be rendered frame by frame
/////////////////////////////////////////////////////////////
triangle_t* triangles_to_render = NULL;

Render_Method render_method =  RENDER_WIRE;
Cull_Method cull_method = CULL_BACKFACE;

/////////////////////////////////////////////////////////////
//// Array of triangle that should be rendered frame by frame
/////////////////////////////////////////////////////////////
bool is_running = false;
int previous_frame_time = 0;

vec3_t camera_position = {.x = 0, .y = 0, .z = 0};
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
  
  load_obj_file_data("./assets/cube.obj");
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
      if (event.key.keysym.sym == SDLK_1) {
        render_method = RENDER_WIRE_VERTEX;
      }
      if (event.key.keysym.sym == SDLK_2) {
        render_method = RENDER_WIRE;
      }
      if (event.key.keysym.sym == SDLK_3) {
        render_method = RENDER_FILL_TRIANGLE;
      }
      if (event.key.keysym.sym == SDLK_4) {
        render_method = RENDER_FILL_TRIANGLE_WIRE;
      }
      if (event.key.keysym.sym == SDLK_c) {
        cull_method = CULL_BACKFACE;
      }
      if (event.key.keysym.sym == SDLK_d) {
        cull_method = CULL_NONE;
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

    vec3_t transformed_vertices[3];

    // Loop all three vertices of this current face
    // and apply transformations
    for (int j = 0; j < 3; j++) {
      vec3_t transformed_vertex = face_vertices[j];

      transformed_vertex = vec3_rotate_x(transformed_vertex, mesh.rotation.x);
      transformed_vertex = vec3_rotate_y(transformed_vertex, mesh.rotation.y);
      transformed_vertex = vec3_rotate_z(transformed_vertex, mesh.rotation.z);

      // Translate the vertex away from the camera
      transformed_vertex.z += 5;

      // Save transformed vertex in the array of the transformed vertices
      transformed_vertices[j] = transformed_vertex;
    }

    // Check the backface culling check 
    if (cull_method == CULL_BACKFACE) {
      vec3_t vector_a = transformed_vertices[0]; // VEC A
      vec3_t vector_b = transformed_vertices[1]; // VEC B
      vec3_t vector_c = transformed_vertices[2]; // VEC C

      vec3_t vector_ab = vec3_sub(vector_b,  vector_a);
      vec3_t vector_ac = vec3_sub(vector_c,  vector_a);

      vec3_t normal = vec3_cross(vector_ab, vector_ac);

      vec3_t camera_ray = vec3_sub(camera_position, vector_a); 

      float dot_product = vec3_dot(normal, camera_ray);

      if (dot_product < 0) continue;
    }
    
    triangle_t projected_triangle;
    // Loop all three vertices to perform projection
    for (int j = 0; j < 3; j++) {
      // Project the current vertex
      vec2_t projected_point = project(transformed_vertices[j]);

      // Scale and translate the project point to
      // the middle of the seen
      projected_point.x += (window_width / 2);
      projected_point.y += (window_height / 2);

      projected_triangle.points[j] = projected_point;
    }

    array_push(triangles_to_render, projected_triangle);
  }
}

/////////////////////////////////////////////////////////////
//// Render function to draw objects on the display
/////////////////////////////////////////////////////////////
void render() {
  clear_color_buffer(0xFF000000);

  // Loop all projected triangles and render them
  int num_triangles = array_length(triangles_to_render);
  for (int i = 0; i < num_triangles; i++) {
    triangle_t triangle = triangles_to_render[i];

    if (render_method == RENDER_FILL_TRIANGLE || render_method == RENDER_FILL_TRIANGLE_WIRE) {
      draw_filled_triangle(
          triangle.points[0].x, triangle.points[0].y,
          triangle.points[1].x, triangle.points[1].y,
          triangle.points[2].x, triangle.points[2].y,
          0xFF555555
      );
    }

    if (render_method == RENDER_WIRE || render_method == RENDER_WIRE_VERTEX || render_method == RENDER_FILL_TRIANGLE_WIRE) {
      draw_triangle(
          triangle.points[0].x, triangle.points[0].y,
          triangle.points[1].x, triangle.points[1].y,
          triangle.points[2].x, triangle.points[2].y,
          0xFFFFFFFF
      );
    }

    if (render_method == RENDER_WIRE_VERTEX) {
      // Draw vertex points
      draw_rect(triangle.points[0].x - 3, triangle.points[0].y - 3, 6, 6, 0xFFFF0000);
      draw_rect(triangle.points[1].x - 3, triangle.points[1].y - 3, 6, 6, 0xFFFF0000);
      draw_rect(triangle.points[2].x - 3, triangle.points[2].y - 3, 6, 6, 0xFFFF0000);
    }
  }

  // Clear array of triangles to render every frame
  array_free(triangles_to_render);

  render_color_buffer();

  SDL_RenderPresent(renderer);
}

/////////////////////////////////////////////////////////////
//// Render function to draw objects on the display
/////////////////////////////////////////////////////////////
void free_resources() {
  free(color_buffer);
  array_free(mesh.faces);
  array_free(mesh.vertices);
}



/////////////////////////////////////////////////////////////
//// Render function to draw objects on the display
/////////////////////////////////////////////////////////////
int main() {
  is_running = initialize_window();

  setup();

  while (is_running) {
    process_input();
    update();
    render();
  }

  destroy_window();
  free_resources();
  
  std::cout << is_running << std::endl;

  return 0;
}
