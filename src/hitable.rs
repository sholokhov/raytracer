use crate::vec;

pub struct HitRecord {
    pub t: f32,
    pub p: vec::Vec3,
    pub normal: vec::Vec3
}

pub trait Hitable {
    fn hit(&self, ray: &vec::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: vec::Vec3,
    pub radius: f32
}

impl Hitable for Sphere {
    fn hit(&self, ray: &vec::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        match discriminant {
            d if d > 0_f32 => {
                let square = (b * b - a * c).sqrt();
                let root1 = (-b - square) / a;
                let root2 = (-b + square) / a;
                let range = t_min..t_max;

                let root =
                    if range.contains(&root1) {
                        Some(root1)
                    } else if range.contains(&root2) {
                        Some(root2)
                    } else {
                        None
                    };

                if let Some(root) = root {
                    let point = ray.point_at_parameter(root);
                    Some(HitRecord {
                        t: root,
                        p: point,
                        normal: (point - self.center) / self.radius
                    })
                } else {
                    None
                }
            },
            _ => None
        }
    }
}

impl Hitable for Vec<Box<dyn Hitable>> {
    fn hit(&self, ray: &vec::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;
        for obj in self {
            if let Some(hit) = obj.hit(&ray, t_min, t_max) {
                closest = match closest {
                    None => Some(hit),
                    Some(ref prev) if hit.t < prev.t => Some(hit),
                    _ => closest
                }
            }
        }
        closest
    }
}