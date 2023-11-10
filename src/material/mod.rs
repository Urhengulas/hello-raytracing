use crate::{hittable::HitRecord, ray::Ray, vec3::Color3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Color3, scattered: &Ray) -> bool;
}

impl Material for () {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _attenuation: &Color3, _scattered: &Ray) -> bool {
        false
    }
}
