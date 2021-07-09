use crate::hittable::{hittable_list::*, sphere::*};
use crate::material::{dielectric::*, lambertian::*, metal::*};
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

pub fn random_scene() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let mut rng = rand::thread_rng();
    world.objects.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            // let choose_mat = ;
            let c1 = (a as f64) + 0.9 * rng.gen_range(0.0..1.0);
            let c3 = (b as f64) + 0.9 * rng.gen_range(0.0..1.0);
            let center = Point3::new(c1, 0.2, c3);

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match rng.gen_range(0.0..1.0) {
                    v if v < 0.8 => {
                        let albedo = random_vec3(0.0, 1.0);
                        let material = Lambertian::new(albedo);
                        world
                            .objects
                            .push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                    v if v < 0.95 => {
                        let albedo = random_vec3(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..0.5);
                        let material = Metal::new(albedo, fuzz);
                        world
                            .objects
                            .push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                    _ => {
                        let material = Dielectric::new(1.5);
                        world
                            .objects
                            .push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                };
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    world
        .objects
        .push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.objects.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    )));

    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world
        .objects
        .push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    world
}
