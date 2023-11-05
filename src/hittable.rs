use std::ops::RangeInclusive;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_range: RangeInclusive<f64>, rec: &mut HitRecord) -> bool;
}

#[derive(Debug, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}
