#[macro_use]
mod utils;
mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::*;
use hittable::*;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use vec3::Point3;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;
    const SAMPLE_PER_PIXEL: i64 = 100;

    // World
    let world = Box::new(HittableList {
        objects: vec![
            Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    }) as Box<dyn Hittable>;

    // Camera
    let camera = Camera::new();

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
                color += ray_color(&r, &world);
            }
            color.write_color(SAMPLE_PER_PIXEL);
        }
    }
}

fn ray_color(r: &Ray, world: &Box<dyn Hittable>) -> Color {
    match world.hit(r, 0.0, f64::MAX) {
        Some(rec) => 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}
