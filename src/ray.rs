use crate::hittable::*;
use crate::vec3::{Color, Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &impl Hittable, depth: i64) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(self, 0.001, f64::MAX) {
            return match rec.material.scatter(self, &rec) {
                None => Color::new(0.0, 0.0, 0.0),
                Some((scattered, attenuation)) => attenuation * scattered.color(world, depth - 1),
            };
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
