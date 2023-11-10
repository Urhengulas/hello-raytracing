mod lambertian;
mod metal;

use crate::{hittable::HitRecord, ray::Ray, vec3::Color3};

pub use self::{lambertian::Lambertian, metal::Metal};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color3, Ray)>;
}

impl Material for () {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color3, Ray)> {
        todo!()
    }
}
