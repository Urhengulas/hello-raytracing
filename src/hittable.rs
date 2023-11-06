use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

#[derive(Debug, Default)]
pub struct HitRecord {
    pub front_face: bool,
    pub normal: Vec3,
    pub p: Point3,
    pub t: f64,
}

impl HitRecord {
    /// Sets the hit record normal vector.
    ///
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }
}
