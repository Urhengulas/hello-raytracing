use crate::vec3::{Color, Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
