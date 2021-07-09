use crate::color::*;
use crate::material::*;
use crate::utils::random::*;
use crate::vec3::*;

#[derive(Clone, Debug, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzzy: f64,
}

impl Metal {
    pub fn new(color: Color, fuzzy: f64) -> Self {
        let f = match fuzzy < 1.0 {
            true => fuzzy,
            false => 1.0,
        };
        Self {
            albedo: color,
            fuzzy: f,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.direction.unit_vector().reflect(&rec.normal);

        let scattered = Ray::new(rec.p, reflected + self.fuzzy * random_in_unit_sphere());

        match Vec3::dot(&scattered.direction, &rec.normal) > 0.0 {
            true => Some((scattered, self.albedo)),
            false => None,
        }
    }
}
