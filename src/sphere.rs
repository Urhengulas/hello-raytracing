use std::ops::RangeInclusive;

use crate::{hittable::{Hittable, HitRecord}, vec3::Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_range: RangeInclusive<f64>,
        rec: &mut HitRecord,
    ) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !ray_range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !ray_range.contains(&root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}
