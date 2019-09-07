use crate::vec::Ray;
use crate::hitable::Hitable;

mod vec;
mod hitable;

fn color(ray: &vec::Ray, world: &Vec<Box<dyn hitable::Hitable>>) -> vec::Vec3 {
    if let Some(hit) = world.hit(ray, 0_f32, std::f32::MAX) {
        0.5 * vec::Vec3(hit.normal.x() + 1f32, hit.normal.y() + 1f32, hit.normal.z() + 1f32)
    } else {
        let unit_direction = ray.direction.to_unit_vector();
        let t = 0.5_f32 * (unit_direction.y() + 1.0);

        (1.0 - t) * vec::Vec3(1_f32, 1_f32, 1_f32) + t * vec::Vec3(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 840;
    let ny = 480;
    let dp = 255;
    println!("P3\n{} {}\n{}", nx, ny, dp);

    let llc = vec::Vec3(-2.0, -1.0, -1.0);
    let hor = vec::Vec3(4.0, 0.0, 0.0);
    let ver = vec::Vec3(0.0, 2.0, 0.0);
    let org = vec::Vec3(0.0, 0.0, 0.0);



    let world: Vec<Box<dyn Hitable>> = vec![
        Box::new(hitable::Sphere {
            center: vec::Vec3(0_f32, 0_f32, -1_f32),
            radius: 0.5
        }),
        Box::new(hitable::Sphere {
            center: vec::Vec3(0_f32, -100.5, -1_f32),
            radius: 100_f32
        }),
    ];

    for j in (0..ny-1).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let ray = Ray::new(org, llc + u * hor + v * ver);
            let col = color(&ray, &world);

            let ir = (255.99 * col.x()) as i32;
            let ig = (255.99 * col.y()) as i32;
            let ib = (255.99 * col.z()) as i32;

            println!("{} {} {}", ir, ig, ib)
        }
    }

}
