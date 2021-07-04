#[macro_use]
mod utils;
mod ray;
mod vec3;

use ray::Ray;
use vec3::Color;

fn main() {
    const IMAGE_WIDTH: i64 = 256;
    const IMAGE_HEIGHT: i64 = 256;

    print!("P3\n{:?} {:?}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = (IMAGE_HEIGHT - j - 1) as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b: f64 = 0.25;

            let color = Color::new(r, g, b);
            color.write_color();
        }
    }
}
