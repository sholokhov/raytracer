mod vec;
mod hitable;
mod camera;
mod material;
mod utils;

use utils::{drand, rand_vec3};
use material::{Lambertian, Metal, Dielectric, Material};
use vec::{Vec3, Ray};
use hitable::{Hitable, Sphere};
use camera::Camera;

fn color(ray: &Ray, world: &Vec<Box<dyn hitable::Hitable>>, depth: i8) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, std::f32::MAX) {
        let scatter = hit.material.scatter(ray, &hit);
        if depth < 50 && scatter.ray.is_some() {
            scatter.attenuation * color(&scatter.ray.unwrap(), world, depth + 1)
        } else {
            Vec3(0_f32, 0_f32, 0_f32)
        }
    } else {
        let unit_direction = ray.direction.to_unit_vector();
        let t = 0.5_f32 * (unit_direction.y() + 1.0);

        (1.0 - t) * Vec3(1_f32, 1_f32, 1_f32) + t * Vec3(0.5, 0.7, 1.0)
    }
}

fn random_scene() -> Box<Vec<Box<dyn Hitable>>> {
    let mut spheres: Vec<Sphere> = vec![
        Sphere {
            center: Vec3(0.0, 0.0, -1000.0),
            radius: 1000.0,
            material: Box::new(Lambertian {
                albedo: Vec3(1.0, 0.6, 0.5)
            })
        },
        Sphere {
            center: Vec3(-4.0, 0.0, 2.0),
            radius: 2.0,
            material: Box::new(Lambertian {
                albedo: Vec3(0.6, 0.2, 0.2)
            })
        },
        Sphere {
            center: Vec3(0.0, 0.0, 2.0),
            radius: 2.0,
            material: Box::new(Dielectric {
                ref_idx: 1.5
            })
        },
        Sphere {
            center: Vec3(4.0, 0.0, 2.0),
            radius: 2.0,
            material: Box::new(Metal {
                albedo: Vec3(0.85, 0.9, 0.7),
                fuzz: 0.0,
            })
        }
    ];

    fn random_material() -> Box<dyn Material> {
        match rand::random::<f32>() {
            0.0 ..= 0.7 =>
                Box::new(Lambertian {
                    albedo: rand_vec3()
                }),
            0.7 ..= 0.9 =>
                Box::new(Metal {
                    albedo: Vec3(0.5, 0.5, 0.5) + 0.5 * rand_vec3(),
                    fuzz: 0.5 * rand::random::<f32>()
                }),
            _ =>
                Box::new(Dielectric {
                    ref_idx: 1.5
                })
        }
    }

    for _ in 0..500 {
        let r = 0.4;
        let Vec3(x, y, _) = utils::random_in_unit_disc();
        let pos = 20.0 * Vec3(x, y, 0.0) + Vec3(0.0, 0.0, r);
        if spheres.iter().all(|s| (s.center - pos).length() >= s.radius + r) {
            spheres.push(Sphere {
                center: pos,
                radius: r,
                material: random_material()
            });
        }
    }

    let world: Vec<Box<dyn Hitable>> = spheres.into_iter().map(|s| Box::new(s) as Box<dyn Hitable>).collect();
    Box::new(world)
}

fn main() {
    let nx = 800;
    let ny = 400;
    let ns = 100;
    let dp = 255;
    println!("P3\n{} {}\n{}", nx, ny, dp);

    let lookfrom = Vec3(20.0 * 0.47f32.cos(), 20.0 * 0.47f32.sin(), 3.0);
    let lookat = Vec3(0.0, 0.0, 1.0);
    let vup = Vec3(0.0, 0.0, 1.0);
    let focus_distance = (lookfrom - lookat).length();
    let aperture = 0.3;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        nx as f32 / ny as f32,
        aperture,
        focus_distance
    );

    let world: Vec<Box<dyn Hitable>> = *random_scene();

    for j in (0..ny-1).rev() {
        for i in 0..nx {
            let mut col = Vec3(0_f32, 0_f32, 0_f32);
            for _ in 0..ns {
                let u = (i as f32 + drand()) / nx as f32;
                let v = (j as f32 + drand()) / ny as f32;
                let ray = camera.ray(u, v);
                col = col + color(&ray, &world, 0);
            }
            col = col / (ns as f32);
            col = Vec3(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

            let ir = (255.99 * col.x()) as i32;
            let ig = (255.99 * col.y()) as i32;
            let ib = (255.99 * col.z()) as i32;

            println!("{} {} {}", ir, ig, ib)
        }
    }

}
