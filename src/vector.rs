#[derive(Clone)]
pub struct Vector2 {
 pub x: f32,
 pub y: f32
}

#[derive(Clone)]
pub struct Vector3 {
 pub x: f32,
 pub y: f32,
 pub z: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
}


impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z}
    }
}
