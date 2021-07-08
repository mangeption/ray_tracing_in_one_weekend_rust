use crate::color::*;
use crate::material::*;
use crate::vec3::*;
use rand::prelude::*;

#[derive(Clone, Debug, Copy)]
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(index_of_reflection: f64) -> Self {
        Dielectric {
            ir: index_of_reflection,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0_squared = r0 * r0;
        r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };

        let unit_direction = r_in.direction.unit_vector();
        let x = Vec3::dot(&-unit_direction, &rec.normal);
        let cos_theta = match x > 1.0 {
            true => 1.0,
            false => x,
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut rng = rand::thread_rng();

        let r_out = match refraction_ratio * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            true => unit_direction.reflect(&rec.normal),
            false => unit_direction.refract(&rec.normal, refraction_ratio),
        };

        Some((Ray::new(rec.p, r_out), attenuation))
    }
}
