use crate::vec3::{Color, Point3, Vec3};

const LIGHT_BLUE: Color = Color::new(0.5, 0.7, 1.0);
const WHITE: Color = Color::new(1.0, 1.0, 1.0);
const RED: Color = Color::new(1.0, 0.0, 0.0);

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
        if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, self) {
            RED
        } else {
            let unit_direction = self.direction.unit_vector();
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * WHITE + a * LIGHT_BLUE
        }
    }
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - *center;
    let a = r.direction.dot(&r.direction);
    let b = 2.0 * oc.dot(&r.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}
