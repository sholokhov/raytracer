use crate::hitable::Hitable;
use crate::camera::Camera;

use utils::{random_in_unit_sphere, drand};
use crate::material::{Lambertian, Metal, Dielectric};

mod vec;
mod hitable;
mod camera;
mod material;
mod utils;

fn color(ray: &vec::Ray, world: &Vec<Box<dyn hitable::Hitable>>, depth: i8) -> vec::Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, std::f32::MAX) {
        let scatter = hit.material.scatter(ray, &hit);
        if depth < 50 && scatter.ray.is_some() {
            scatter.attenuation * color(&scatter.ray.unwrap(), world, depth + 1)
        } else {
            vec::Vec3(0_f32, 0_f32, 0_f32)
        }
    } else {
        let unit_direction = ray.direction.to_unit_vector();
        let t = 0.5_f32 * (unit_direction.y() + 1.0);

        (1.0 - t) * vec::Vec3(1_f32, 1_f32, 1_f32) + t * vec::Vec3(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let dp = 255;
    println!("P3\n{} {}\n{}", nx, ny, dp);

    let r = (std::f32::consts::PI / 4_f32).cos();
    let camera = Camera::new(
        vec::Vec3(0.0, 0.0, 0.0),
        vec::Vec3(0.0, 0.0, -2.0),
        vec::Vec3(0.0, 1.0, 0.0),
        120.0,
        nx as f32 / ny as f32
    );

    let world: Vec<Box<dyn Hitable>> = vec![
        Box::new(hitable::Sphere {
            center: vec::Vec3(0_f32, 0_f32, -1_f32),
            radius: 0.5,
            material: Box::new(Lambertian {
                albedo: vec::Vec3(0.8, 0.3, 0.3)
            })
        }),
        Box::new(hitable::Sphere {
            center: vec::Vec3(0_f32, -100.5, -1_f32),
            radius: 100_f32,
            material: Box::new(Lambertian {
                albedo: vec::Vec3(0.8, 0.8, 0.0)
            })
        }),
        Box::new(hitable::Sphere {
            center: vec::Vec3(1_f32, 0_f32, -1_f32),
            radius: 0.5,
            material: Box::new(Metal {
                albedo: vec::Vec3(0.8, 0.6, 0.2),
                fuzz: 0.3
            })
        }),
        Box::new(hitable::Sphere {
            center: vec::Vec3(-1_f32, 0_f32, -1_f32),
            radius: 0.5,
            material: Box::new(Dielectric {
                ref_idx: 1.5
            })
        }),
        Box::new(hitable::Sphere {
            center: vec::Vec3(-1_f32, 0_f32, -1_f32),
            radius: -0.45,
            material: Box::new(Dielectric {
                ref_idx: 1.5
            })
        }),
    ];

    for j in (0..ny-1).rev() {
        for i in 0..nx {
            let mut col = vec::Vec3(0_f32, 0_f32, 0_f32);
            for _ in 0..ns {
                let u = (i as f32 + drand()) / nx as f32;
                let v = (j as f32 + drand()) / ny as f32;
                let ray = camera.ray(u, v);
                col = col + color(&ray, &world, 0);
            }
            col = col / (ns as f32);
            col = vec::Vec3(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

            let ir = (255.99 * col.x()) as i32;
            let ig = (255.99 * col.y()) as i32;
            let ib = (255.99 * col.z()) as i32;

            println!("{} {} {}", ir, ig, ib)
        }
    }

}
