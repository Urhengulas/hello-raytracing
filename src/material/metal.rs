use crate::{hittable::HitRecord, material::Material, ray::Ray, vec3::Color3};

struct Metal {
    albedo: Color3,
}

impl Metal {
    fn new(albedo: Color3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color3, Ray)> {
        let reflected = r_in.direction.unit_vector().reflect(&rec.normal);

        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
