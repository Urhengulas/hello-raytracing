mod ray;
mod vec3;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

fn main() {
    // Image

    let image_width = 1024;
    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width * 16 / 9).max(1);

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = {
        let image_width = Into::<f64>::into(image_width);
        let image_height = Into::<f64>::into(image_height);
        viewport_height * (image_width / image_height)
    };
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / Into::<f64>::into(image_width);
    let pixel_delta_v = viewport_v / Into::<f64>::into(image_height);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {:05}", image_height - j);
        for i in 0..image_width {
            let i: f64 = i.try_into().unwrap();
            let j: f64 = j.try_into().unwrap();

            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = r.ray_color();
            pixel_color.write_color();
        }
    }
    eprintln!("\rDone.{}", " ".repeat(25));
}
