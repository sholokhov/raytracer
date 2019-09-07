use crate::vec;

pub fn drand() -> f32 {
    rand::random::<f32>().abs()
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
