use crate::hittable::*;
use crate::ray::*;
use crate::vec3::Color;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}
