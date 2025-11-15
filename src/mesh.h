#ifndef MESH_H
#define MESH_H

#include "triangle.h"
#include "vector.h"
#include <string>
#define N_CUBE_VERTICES 8
#define N_CUBE_FACES (6 * 2)

extern vec3_t cube_vertices[N_CUBE_VERTICES];
extern face_t cube_faces[N_CUBE_FACES];

/////////////////////////////////////////////
//// Define a struct for dynamic size meshes,
//// with array of vertices and faces
/////////////////////////////////////////////

typedef struct {
  vec3_t* vertices;
  face_t* faces;

  vec3_t rotation;
  vec3_t scale;
  vec3_t translation;
} mesh_t;

extern mesh_t mesh;

void load_cube_mesh_data();
void load_obj_file_data(std::string filename);

#endif // MESH_H
