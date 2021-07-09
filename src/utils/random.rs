use crate::vec3::*;
use rand::prelude::*;

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();

    match Vec3::dot(&in_unit_sphere, normal) > 0.0 {
        true => in_unit_sphere,
        false => -in_unit_sphere,
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_vec3(min: f64, max: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
