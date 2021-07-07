use crate::color::*;
use crate::hittable::*;
use crate::ray::*;

pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}
