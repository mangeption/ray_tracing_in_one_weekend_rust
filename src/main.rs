#[macro_use]
mod utils;
mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use rand::prelude::*;
use ray::Ray;
use utils::random::random_scene;
use vec3::*;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i64 = 1200;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;
    const SAMPLE_PER_PIXEL: i64 = 500;
    const MAX_DEPTH: i64 = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
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
                color += r.color(&world, MAX_DEPTH);
            }
            color.write_color(SAMPLE_PER_PIXEL);
        }
    }
}
