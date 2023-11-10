mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod util;
mod vec3;

use std::rc::Rc;

use vec3::Vec3;

use crate::{
    camera::Camera,
    hittable::{HittableList, Sphere},
    material::{Dielectric, Lambertian, Metal},
    vec3::{Color3, Point3},
};

const ASPECT_RATIO: f64 = 9. / 16.;
const IMAGE_WIDTH: u32 = 1024;
const LOOKAT: Point3 = Point3::new(0., 0., -1.);
const LOOKFROM: Point3 = Point3::new(-2., 2., 1.);
const MAX_DEPTH: u32 = 50;
const SAMPLES_PER_PIXEL: u32 = 100;
const VFOV: f64 = 20.;
const VUP: Vec3 = Vec3::new(0., 1., 0.);

fn main() {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color3::new(0.8, 0.8, 0.)));
    let material_center = Rc::new(Lambertian::new(Color3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color3::new(0.8, 0.6, 0.2), 0.));

    world.add(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    ));
    world.add(Sphere::new(Point3::new(0., 0., -1.), 0.5, material_center));
    world.add(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    ));
    world.add(Sphere::new(Point3::new(-1., 0., -1.), -0.4, material_left));
    world.add(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right));

    let cam = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        LOOKAT,
        LOOKFROM,
        MAX_DEPTH,
        SAMPLES_PER_PIXEL,
        VFOV,
        VUP,
    );
    cam.render(&world);
}
