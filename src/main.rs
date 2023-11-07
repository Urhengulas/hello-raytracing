mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod util;
mod vec3;

use crate::{camera::Camera, hittable_list::HittableList, sphere::Sphere, vec3::Point3};

fn main() {
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0., 0., -1.), 0.5));
    world.add(Sphere::new(Point3::new(0., -100.5, -1.), 100.));

    let cam = Camera::new(9. / 16., 1024, 100);
    cam.render(&world);
}
