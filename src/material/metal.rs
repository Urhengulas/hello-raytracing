use crate::{
    hittable::HitRecord,
    material::Material,
    ray::Ray,
    vec3::{Color3, Vec3},
};

pub struct Metal {
    albedo: Color3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color3, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color3, Ray)> {
        let reflected = r_in.direction.unit_vector().reflect(&rec.normal);

        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_unit_vector());
        let attenuation = self.albedo;
        (scattered.direction.dot(&rec.normal) > 0.).then_some((attenuation, scattered))
    }
}
