#[macro_use]
mod utils;
mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::*;
use hittable::*;
use material::{dielectric::*, lambertian::*, metal::*};
use rand::prelude::*;
use ray::Ray;
use sphere::*;
use vec3::*;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;
    const SAMPLE_PER_PIXEL: i64 = 100;
    const MAX_DEPTH: i64 = 50;

    // World
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    let world = HittableList {
        objects: vec![
            Box::new(Sphere::new(
                Point3::new(0.0, -100.5, -1.0),
                100.0,
                material_ground,
            )),
            Box::new(Sphere::new(
                Point3::new(0.0, 0.0, -1.0),
                0.5,
                material_center,
            )),
            Box::new(Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),
                0.5,
                material_left,
            )),
            Box::new(Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),
                -0.4,
                material_left,
            )),
            Box::new(Sphere::new(
                Point3::new(1.0, 0.0, -1.0),
                0.5,
                material_right,
            )),
        ],
    };

    // Camera
    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render
    let mut rng = rand::thread_rng();

    println!("P3\n{:?} {:?}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLE_PER_PIXEL {
                let u = ((i as f64) + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f64;
                let v = (((IMAGE_HEIGHT - j - 1) as f64) + rng.gen_range(0.0..1.0))
                    / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                color += ray_color(&r, &world, MAX_DEPTH);
            }
            color.write_color(SAMPLE_PER_PIXEL);
        }
    }
}

fn ray_color(r: &Ray, world: &impl Hittable, depth: i64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::MAX) {
        return match rec.material.scatter(r, &rec) {
            None => Color::new(0.0, 0.0, 0.0),
            Some((scattered, attenuation)) => attenuation * ray_color(&scattered, world, depth - 1),
        };
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
