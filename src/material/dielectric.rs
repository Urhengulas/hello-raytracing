use crate::{hittable::HitRecord, material::Material, ray::Ray, util::random_double, vec3::Color3};

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
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let schlick = reflectance(cos_theta, refraction_ratio) > random_double();
        let direction = match cannot_refract || schlick {
            true => unit_direction.reflect(&rec.normal),
            false => unit_direction.refract(&rec.normal, refraction_ratio),
        };

        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }
}

/// Schlick's approximation for reflectance
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
