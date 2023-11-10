mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod util;
mod vec3;

use std::rc::Rc;

use crate::{
    camera::Camera,
    hittable::{HittableList, Sphere},
    vec3::Point3,
};

fn main() {
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0., 0., -1.), 0.5, Rc::new(())));
    world.add(Sphere::new(Point3::new(0., -100.5, -1.), 100., Rc::new(())));

    let cam = Camera::new(9. / 16., 1024, 100, 50);
    cam.render(&world);
}
