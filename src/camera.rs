use crate::vec;
use crate::utils;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: vec::Vec3,
    vertical: vec::Vec3,
    horizontal: vec::Vec3,
    lower_left_corner: vec::Vec3,
    u: vec::Vec3,
    v: vec::Vec3,
    w: vec::Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn new(lookfrom: vec::Vec3, lookat: vec::Vec3, vup: vec::Vec3,
               vfov: f32, aspect: f32, aperture: f32, focus_distance: f32) -> Camera
    {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).to_unit_vector();
        let u = vup.cross(w).to_unit_vector();
        let v = w.cross(u);

        Camera {
            origin: lookfrom,
            u, v, w,
            lower_left_corner: lookfrom - focus_distance * (half_width * u + half_height * v + w),
            horizontal: focus_distance * 2.0 * half_width * u,
            vertical: focus_distance * 2.0 * half_height * v,
            lens_radius: aperture / 2.0
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> vec::Ray {
        let vec::Vec3(du, dv, _) = self.lens_radius * utils::random_in_unit_disc();
        let origin = self.origin + du * self.u + dv * self.v;
        vec::Ray::new(
            origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - origin
        )
    }
}