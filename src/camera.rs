use crate::vec;

pub struct Camera {
    origin: vec::Vec3,
    vertical: vec::Vec3,
    horizontal: vec::Vec3,
    lower_left_corner: vec::Vec3,
}

impl Camera {
    pub fn new(lookfrom: vec::Vec3, lookat: vec::Vec3,
               vup: vec::Vec3, vfov: f32, aspect: f32) -> Camera
    {
        let theta = vfov * std::f32::consts::PI / 180_f32;
        let half_height = (theta / 2_f32).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).to_unit_vector();
        let u = vup.cross(w).to_unit_vector();
        let v = w.cross(u);

        Camera {
            origin: lookfrom,
            vertical: 2.0 * half_height * v,
            horizontal: 2.0 * half_width * u,
            lower_left_corner: lookfrom - half_width * u - half_height * v - w,
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> vec::Ray {
        vec::Ray {
            origin: self.origin,
            direction: self.lower_left_corner +
                u * self.horizontal + v * self.vertical,
        }
    }
}