#[derive(Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
}

impl Vector3 {
    /// Creates a new Vec3 object with the given x, y and z points
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    /// Rotates around the x-axis
    pub fn rotate_x(&self, beta_angle: f32) -> Vector3 {
        Vector3::new(
            self.x,
            self.y * f32::cos(beta_angle) - self.z * f32::sin(beta_angle),
            self.y * f32::sin(beta_angle) + self.z * f32::cos(beta_angle),
        )
    }

    /// Rotates around the y-axis
    pub fn rotate_y(&self, beta_angle: f32) -> Vector3 {
        Vector3::new(
            self.x * f32::cos(beta_angle) - self.z * f32::sin(beta_angle),
            self.y, 
            self.x * f32::sin(beta_angle) + self.z * f32::cos(beta_angle),
        )
    }

    /// Rotates around the z-axis
    pub fn rotate_z(&self, beta_angle: f32) -> Vector3 {
        Vector3::new(
            self.x * f32::cos(beta_angle) - self.y * f32::sin(beta_angle),
            self.x * f32::sin(beta_angle) + self.y * f32::cos(beta_angle),
            self.z 
        )
    }
}
