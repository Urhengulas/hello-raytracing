use crate::vec3::{Color, Point3, Vec3};

const LIGHT_BLUE: Color = Color::new(0.5, 0.7, 1.);
const WHITE: Color = Color::new(1., 1., 1.);
const RED: Color = Color::new(1., 0., 0.);

pub struct Ray {
    origin: Point3,
    direction: Vec3,
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
        let t = hit_sphere(&center, 0.5, self);
        if t > 0. {
            let n = (self.at(t) - center).unit_vector();
            0.5 * (n + 1.)
        } else {
            let unit_direction = self.direction.unit_vector();
            let a = 0.5 * (unit_direction.y + 1.);
            (1. - a) * WHITE + a * LIGHT_BLUE
        }
    }
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    match discriminant < 0. {
        true => -1.,
        false => (-half_b - discriminant.sqrt()) / a,
    }
}
