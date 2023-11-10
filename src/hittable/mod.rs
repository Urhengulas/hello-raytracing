mod hittable_list;
mod sphere;

use std::rc::Rc;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub use self::{hittable_list::HittableList, sphere::Sphere};

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub front_face: bool,
    pub material: Rc<dyn Material>,
    pub normal: Vec3,
    pub p: Point3,
    pub t: f64,
}

impl HitRecord {
    pub fn new(material: Rc<dyn Material>, p: Point3, t: f64) -> Self {
        Self {
            material,
            p,
            t,
            // will be set with .set_face_normal
            front_face: Default::default(),
            normal: Default::default(),
        }
    }

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
