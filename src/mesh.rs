use std::i32;

use crate::{triangle::Face, vector::Vector3};

pub const N_MESH_VERTICES: i32 = 8;
pub static mut MESH_VERTICES: [Vector3; N_MESH_VERTICES as usize] = [
    Vector3 {x: -1.0,y: -1.0,z: -1.0},
    Vector3 {x: -1.0,y:  1.0,z: -1.0},
    Vector3 {x:  1.0,y:  1.0,z: -1.0},
    Vector3 {x:  1.0,y: -1.0,z: -1.0},
    Vector3 {x:  1.0,y:  1.0,z:  1.0},
    Vector3 {x:  1.0,y: -1.0,z:  1.0},
    Vector3 {x: -1.0,y:  1.0,z:  1.0},
    Vector3 {x: -1.0,y: -1.0,z:  1.0},
];

pub const N_MESH_FACES: i32 = 6 * 2;
pub static mut MESH_FACES: [Face; N_MESH_FACES as usize] = [
    // FRONT
    Face {a: 1, b: 2, c: 3},
    Face {a: 1, b: 3, c: 4},
    // RIGHT
    Face {a: 4, b: 3, c: 5},
    Face {a: 4, b: 5, c: 6},
    // BACK
    Face {a: 6, b: 5, c: 7},
    Face {a: 6, b: 7, c: 8},
    // LEFT
    Face {a: 8, b: 7, c: 2},
    Face {a: 8, b: 2, c: 1},
    // TOP 
    Face {a: 2, b: 7, c: 5},
    Face {a: 2, b: 5, c: 3},
    // BOTTOM
    Face {a: 6, b: 8, c: 1},
    Face {a: 6, b: 1, c: 4},
];
