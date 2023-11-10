use crate::{hittable::HitRecord, material::Material, ray::Ray, vec3::Color3};

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color3, Ray)> {
        let attenuation = Color3::new(1.0, 1.0, 1.0);
        let refraction_ratio = match rec.front_face {
            true => 1. / self.index_of_refraction,
            false => self.index_of_refraction,
        };

        let unit_direction = r_in.direction.unit_vector();
        let refracted = unit_direction.refract(&rec.normal, refraction_ratio);

        let scattered = Ray::new(rec.p, refracted);
        Some((attenuation, scattered))
    }
}
