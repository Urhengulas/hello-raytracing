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
    material::{Dielectric, Lambertian, Metal},
    vec3::{Color3, Point3},
};

const ASPECT_RATIO: f64 = 9. / 16.;
const IMAGE_WIDTH: u32 = 1024;
const VFOV: f64 = 90.;

fn main() {
    let mut world = HittableList::new();

    let r = (std::f64::consts::PI / 4.).cos();

    let material_left = Rc::new(Lambertian::new(Color3::new(0., 0., 1.0)));
    let material_right = Rc::new(Lambertian::new(Color3::new(1., 0., 0.)));

    world.add(Sphere::new(Point3::new(-r, 0., -1.), r, material_left));
    world.add(Sphere::new(Point3::new(r, 0., -1.), r, material_right));

    let cam = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100, 50, VFOV);
    cam.render(&world);
}
