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
    material::{Lambertian, Metal},
    vec3::{Color3, Point3},
};

fn main() {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color3::new(0.7, 0.3, 0.0)));
    let material_left = Rc::new(Metal::new(Color3::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color3::new(0.8, 0.6, 0.2)));

    world.add(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    ));
    world.add(Sphere::new(Point3::new(0., 0., -1.), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1., 0., -1.), 0.5, material_left));
    world.add(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right));

    let cam = Camera::new(9. / 16., 1024, 100, 50);
    cam.render(&world);
}
