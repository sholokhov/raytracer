use crate::vec;
use crate::hitable;
use crate::utils;

use utils::{random_in_unit_sphere};

#[derive(Clone, Copy)]
pub struct Scatter {
    pub attenuation: vec::Vec3,
    pub ray: Option<vec::Ray>
}

pub trait Material {
    fn scatter(&self, r_in: &vec::Ray, hit: &hitable::HitRecord) -> Scatter;
}

pub struct Lambertian {
    pub albedo: vec::Vec3
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &vec::Ray, hit: &hitable::HitRecord) -> Scatter {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        Scatter {
            attenuation: self.albedo,
            ray: Some(vec::Ray::new(hit.p, target - hit.p))
        }
    }
}

fn reflect(v: vec::Vec3, n: vec::Vec3) -> vec::Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub struct Metal {
    pub albedo: vec::Vec3,
    pub fuzz: f32
}

impl Material for Metal {
    fn scatter(&self, r_in: &vec::Ray, hit: &hitable::HitRecord) -> Scatter {
        let reflected = reflect(r_in.direction, hit.normal);
        let scattered = vec::Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
        Scatter {
            attenuation: self.albedo,
            ray: match scattered.direction.dot(hit.normal) {
                x if x <= 0.0 => None,
                _ => Some(scattered)
            }
        }
    }
}