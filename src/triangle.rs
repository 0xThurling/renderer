use crate::vector::Vector2;

#[derive(Clone, Copy)]
pub struct Face {
    pub a: i32,
    pub b: i32,
    pub c: i32
}

#[derive(Clone)]
pub struct Triangle {
    pub points: [Vector2; 3]
}

// TODO: Create implementation
