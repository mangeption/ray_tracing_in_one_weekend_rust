use crate::utils::nums::*;
use crate::vec3::*;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, samples_per_pixel: i64) {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = 256.0 * clamp(self.x() * scale, 0.0, 0.999);
        let g = 256.0 * clamp(self.y() * scale, 0.0, 0.999);
        let b = 256.0 * clamp(self.z() * scale, 0.0, 0.999);

        println!("{:?} {:?} {:?}", r as i64, g as i64, b as i64);
    }
}
