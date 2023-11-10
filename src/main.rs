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

const ASPECT_RATIO: f64 = 9. / 16.;
const IMAGE_WIDTH: u32 = 1024;

fn main() {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color3::new(0.7, 0.3, 0.0)));
    let material_left = Rc::new(Metal::new(Color3::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    ));
    world.add(Sphere::new(Point3::new(0., 0., -1.), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1., 0., -1.), 0.5, material_left));
    world.add(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right));

    let cam = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100, 50);
    cam.render(&world);
}
