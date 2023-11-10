use crate::{
    hittable::HitRecord,
    material::Material,
    ray::Ray,
    vec3::{Color3, Vec3},
};

struct Lambertian {
    albedo: Color3,
}

impl Lambertian {
    fn new(albedo: Color3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color3, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
