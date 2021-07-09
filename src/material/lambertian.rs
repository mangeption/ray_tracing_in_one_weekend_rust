use crate::color::*;
use crate::material::*;
use crate::utils::random::*;

#[derive(Clone, Debug, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = rec.normal + random_in_hemisphere(&rec.normal).unit_vector();

        let d = match scatter_direction.near_zero() {
            true => rec.normal,
            false => scatter_direction,
        };

        Some((Ray::new(rec.p, d), self.albedo))
    }
}
