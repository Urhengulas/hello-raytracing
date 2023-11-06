use crate::{
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vec3, LIGHT_BLUE, WHITE},
};

#[derive(Debug, Default)]
pub struct Camera {
    center: Point3,
    image_height: u32,
    image_width: u32,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    /// Create a new camera.
    ///
    /// - `aspect_ratio`: Ratio of image width over height
    /// - `image_width`: Rendered image width in pixel count
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        let image_width = Into::<f64>::into(image_width);
        let image_height = (image_width * aspect_ratio).max(1.);

        let center = Point3::new(0., 0., 0.);

        // Determine viewport dimensions.
        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (image_width / image_height);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / Into::<f64>::into(image_width);
        let pixel_delta_v = viewport_v / Into::<f64>::into(image_height);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // cast dimenstions to integer
        let image_height = image_height.trunc() as _;
        let image_width = image_width.trunc() as _;

        Self {
            center,
            image_height,
            image_width,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {:05}", self.image_height - j);
            for i in 0..self.image_width {
                let i: f64 = i.try_into().unwrap();
                let j: f64 = j.try_into().unwrap();

                let pixel_center =
                    self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&r, world);
                pixel_color.write_color();
            }
        }
        eprintln!("\rDone.{}", " ".repeat(25));
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(r, &Interval::new(0., f64::INFINITY)) {
            0.5 * (rec.normal + WHITE)
        } else {
            let unit_direction = r.direction.unit_vector();
            let a = 0.5 * (unit_direction.y + 1.);
            (1. - a) * WHITE + a * LIGHT_BLUE
        }
    }
}