mod dielectric;
mod lambertian;
mod metal;

use crate::{hittable::HitRecord, ray::Ray, vec3::Color3};

pub use self::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color3, Ray)>;
}
