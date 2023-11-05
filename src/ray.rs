use crate::vec3::{Point3, Vec3};

struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    fn at(&self, t: f64) -> Point3 {
        self.origin.clone() + self.direction.clone() * t
    }
}
