use crate::vec::Ray;

mod vec;

fn hit_sphere(center: &vec::Vec3, radius: f32, ray: &vec::Ray) -> f32 {
    let oc = ray.origin - (*center);
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;
    match discriminant {
        _ if discriminant < 0.0 => -1.0,
        _ => (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(ray: &vec::Ray) -> vec::Vec3 {
    let v = vec::Vec3(0.0,0.0,-1.0);
    let t = hit_sphere(&v, 0.5, ray);
    if t > 0.0 {
        let normal = (ray.point_at_parameter(t) - v).to_unit_vector();
        return 0.5 * vec::Vec3(normal.x() + 1f32, normal.y() + 1f32, normal.z() + 1f32);
    }

    let unit_direction = ray.direction.to_unit_vector();
    let t = 0.5_f32 * (unit_direction.y() + 1.0);

    (1.0 - t) * vec::Vec3(1_f32,1_f32,1_f32) + t * vec::Vec3(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let dp = 255;

    println!("P3\n{} {}\n{}", nx, ny, dp);
    let llc = vec::Vec3(-2.0, -1.0, -1.0);
    let hor = vec::Vec3(4.0, 0.0, 0.0);
    let ver = vec::Vec3(0.0, 2.0, 0.0);
    let org = vec::Vec3(0.0, 0.0, 0.0);

    for j in (0..ny-1).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let ray = Ray::new(org, llc + u * hor + v * ver);
            let col = color(&ray);

            let ir = (255.99 * col.x()) as i32;
            let ig = (255.99 * col.y()) as i32;
            let ib = (255.99 * col.z()) as i32;

            println!("{} {} {}", ir, ig, ib)
        }
    }

}
