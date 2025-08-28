use std::{
    i32,
    sync::{LazyLock, Mutex},
};

use crate::{triangle::Face, vector::Vector3};

pub const N_CUBE_VERTICES: i32 = 8;
pub const CUBE_VERTICES: [Vector3; N_CUBE_VERTICES as usize] = [
    Vector3 {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    },
    Vector3 {
        x: -1.0,
        y: 1.0,
        z: -1.0,
    },
    Vector3 {
        x: 1.0,
        y: 1.0,
        z: -1.0,
    },
    Vector3 {
        x: 1.0,
        y: -1.0,
        z: -1.0,
    },
    Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
    Vector3 {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    },
    Vector3 {
        x: -1.0,
        y: 1.0,
        z: 1.0,
    },
    Vector3 {
        x: -1.0,
        y: -1.0,
        z: 1.0,
    },
];

pub const N_CUBE_FACES: i32 = 6 * 2;
pub const CUBE_FACES: [Face; N_CUBE_FACES as usize] = [
    // FRONT
    Face { a: 1, b: 2, c: 3 },
    Face { a: 1, b: 3, c: 4 },
    // RIGHT
    Face { a: 4, b: 3, c: 5 },
    Face { a: 4, b: 5, c: 6 },
    // BACK
    Face { a: 6, b: 5, c: 7 },
    Face { a: 6, b: 7, c: 8 },
    // LEFT
    Face { a: 8, b: 7, c: 2 },
    Face { a: 8, b: 2, c: 1 },
    // TOP
    Face { a: 2, b: 7, c: 5 },
    Face { a: 2, b: 5, c: 3 },
    // BOTTOM
    Face { a: 6, b: 8, c: 1 },
    Face { a: 6, b: 1, c: 4 },
];

// ==============================================

pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub faces: Vec<Face>,
    pub rotation: Vector3
}

impl Mesh {
    pub fn load_cube_mesh_data(&mut self) {
        for i in 0..N_CUBE_VERTICES {
            self.vertices.push(CUBE_VERTICES[i as usize].clone());
        }

        for i in 0..N_CUBE_FACES {
            self.faces.push(CUBE_FACES[i as usize]);
        }
    }
}

// My global mesh variable going to be used
pub static MESH: LazyLock<Mutex<Mesh>> = LazyLock::new(|| {
    Mutex::new(Mesh {
        vertices: vec![],                               // dynamic array for our vertice points of our mesh
        faces: vec![],                                  // dynamic array for our faces
        rotation: Vector3 { x: 0.0, y: 0.0, z: 0.0 }    // rotation information, by how much and in
                                                        // which direction the mesh with rotate
    })
});
