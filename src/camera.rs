use crate::vec;

pub struct Camera {
    pub origin: vec::Vec3,
    pub vertical: vec::Vec3,
    pub horizontal: vec::Vec3,
    pub lower_left_corner: vec::Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: vec::Vec3(0.0, 0.0, 0.0),
            vertical: vec::Vec3(0.0, 2.0, 0.0),
            horizontal: vec::Vec3(4.0, 0.0, 0.0),
            lower_left_corner: vec::Vec3(-2.0, -1.0, -1.0),
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> vec::Ray {
        vec::Ray {
            origin: self.origin,
            direction: self.lower_left_corner +
                u * self.horizontal + v * self.vertical
        }
    }
}