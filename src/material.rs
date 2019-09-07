use crate::vec;
use crate::hitable;
use crate::utils;

use utils::{random_in_unit_sphere};
use crate::vec::{Vec3, Ray};
use crate::hitable::HitRecord;

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

fn refract(v: &vec::Vec3, n: &vec::Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.to_unit_vector();
    let dt = uv.dot(*n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    match discriminant {
        _ if discriminant > 0.0 => {
            Some(ni_over_nt * (uv - dt * (*n)) - discriminant.sqrt() * (*n))
        },
        _ => None
    }
}

pub struct Dielectric {
    pub ref_idx: f32
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &vec::Ray, hit: &HitRecord) -> Scatter {
        let att = Vec3(1.0, 1.0, 1.0);
        let reflected = reflect(r_in.direction, hit.normal);

        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;

        if r_in.direction.dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction.dot(hit.normal) / r_in.direction.length();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction.dot(hit.normal) / r_in.direction.length();
        }

        match refract(&r_in.direction, &outward_normal, ni_over_nt) {
            Some(refracted) if rand::random::<f32>() > schlick(cosine, self.ref_idx)  => {
                Scatter {
                    attenuation: att,
                    ray: Some(Ray::new(hit.p, refracted))
                }
            },
            _ => {
                Scatter {
                    attenuation: att,
                    ray: Some(Ray::new(hit.p, reflected))
                }
            }
        }
    }
}