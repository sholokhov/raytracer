use crate::vec;

pub fn drand() -> f32 {
    rand::random::<f32>().abs()
}

pub fn rand_vec3() -> vec::Vec3 {
    vec::Vec3(
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    )
}

pub fn random_in_unit_sphere() -> vec::Vec3 {
    let mut p;
    loop {
        p = 2.0 * vec::Vec3(drand(), drand(), drand()) - vec::Vec3(1f32, 1f32, 1f32);
        if p.squared_length() >= 1.0 {
            break;
        }
    }
    return p;
}

pub fn random_in_unit_disc() -> vec::Vec3 {
    loop {
        let p = vec::Vec3(2.0 * rand::random::<f32>() - 1.0, 2.0 * rand::random::<f32>() - 1.0, 0.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}