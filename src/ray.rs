use std::ops::RangeInclusive;

use crate::{
    hittable::{HitRecord, Hittable},
    sphere::Sphere,
    vec3::{Color, Point3, Vec3},
};

const LIGHT_BLUE: Color = Color::new(0.5, 0.7, 1.);
const WHITE: Color = Color::new(1., 1., 1.);
const RED: Color = Color::new(1., 0., 0.);

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self) -> Color {
        let center = Point3::new(0., 0., -1.);
        let sphere = Sphere::new(center, 0.5);
        let mut rec = HitRecord::default();
        sphere.hit(self, RangeInclusive::new(-1., 1.), &mut rec);
        if rec.t > 0. {
            let n = (self.at(rec.t) - center).unit_vector();
            0.5 * (n + 1.)
        } else {
            let unit_direction = self.direction.unit_vector();
            let a = 0.5 * (unit_direction.y + 1.);
            (1. - a) * WHITE + a * LIGHT_BLUE
        }
    }
}
