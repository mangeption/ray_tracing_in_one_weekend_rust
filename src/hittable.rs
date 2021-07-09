use crate::material::*;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub mod hittable_list;
pub mod sphere;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
