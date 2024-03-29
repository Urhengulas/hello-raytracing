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
    material::{Dielectric, Lambertian, Material, Metal},
    util::{f64, random_double, random_double_minmax},
    vec3::{Color3, Point3, Vec3},
};

const ASPECT_RATIO: f64 = 9. / 16.;
const DEFOCUS_ANGLE: f64 = 0.6;
const FOCUS_DIST: f64 = 10.;
const IMAGE_WIDTH: u32 = 120;
const LOOKAT: Point3 = Point3::new(0., 0., 0.);
const LOOKFROM: Point3 = Point3::new(13., 2., 3.);
const MAX_DEPTH: u32 = 50;
const SAMPLES_PER_PIXEL: u32 = 500;
const VFOV: f64 = 20.;
const VUP: Vec3 = Vec3::new(0., 1., 0.);

pub fn main() {
    let world = make_world();
    let cam = camera();
    cam.render(&world)
}

pub fn make_world() -> HittableList<'static> {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color3::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        material_ground,
    ));

    for a in -2..2 {
        for b in -2..2 {
            let choose_mat = random_double();
            let center = Point3::new(
                f64(a) + 0.9 * random_double(),
                0.2,
                f64(b) + 0.9 * random_double(),
            );

            let sphere_material: Rc<dyn Material> = if choose_mat < 0.9 {
                // diffuse
                let albedo = Color3::random() * Color3::random();
                Rc::new(Lambertian::new(albedo))
            } else if choose_mat < 0.95 {
                // metal
                let albedo = Color3::random_minmax(0.5, 1.);
                let fuzz = random_double_minmax(0., 0.5);
                Rc::new(Metal::new(albedo, fuzz))
            } else {
                // glass
                Rc::new(Dielectric::new(1.5))
            };
            world.add(Sphere::new(center, 0.2, sphere_material));
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    let material2 = Rc::new(Lambertian::new(Color3::new(0.4, 0.2, 0.1)));
    let material3 = Rc::new(Metal::new(Color3::new(0.7, 0.6, 0.5), 0.));

    world.add(Sphere::new(Point3::new(0., 1., 0.), 1., material1));
    world.add(Sphere::new(Point3::new(-4., 1., 0.), 1., material2));
    world.add(Sphere::new(Point3::new(4., 1., 0.), 1., material3));

    world
}

pub fn camera() -> Camera {
    let cam = Camera::new(
        ASPECT_RATIO,
        DEFOCUS_ANGLE,
        FOCUS_DIST,
        IMAGE_WIDTH,
        LOOKAT,
        LOOKFROM,
        MAX_DEPTH,
        SAMPLES_PER_PIXEL,
        VFOV,
        VUP,
    );
    cam
}
