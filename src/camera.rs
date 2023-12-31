use crate::{
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    util::random_double,
    vec3::{Color3, Point3, Vec3, LIGHT_BLUE, WHITE},
};

#[derive(Debug, Default)]
pub struct Camera {
    center: Point3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    image_height: u32,
    image_width: u32,
    max_depth: u32,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
}

impl Camera {
    /// Create a new camera.
    ///
    /// - `aspect_ratio`: Ratio of image width over height
    /// - `defocus_angle`: Variation angle of rays through each pixel
    /// - `focus_dist`: Distance from camera lookfrom point to plane of perfect focus
    /// - `image_width`: Rendered image width in pixel count
    /// - `lookat`: Point camera is looking at
    /// - `lookfrom`: Point camera is looking from
    /// - `max_depth`: Maximum number of ray bounces into scene
    /// - `samples_per_pixel`: Count of random samples for each pixel
    /// - `vfov`: Vertical view angle (field of view)
    /// - `vup`: Camera-relative "up" direction
    pub fn new(
        aspect_ratio: f64,
        defocus_angle: f64,
        focus_dist: f64,
        image_width: u32,
        lookat: Point3,
        lookfrom: Point3,
        max_depth: u32,
        samples_per_pixel: u32,
        vfov: f64,
        vup: Vec3,
    ) -> Self {
        let image_width = Into::<f64>::into(image_width);
        let image_height = (image_width * aspect_ratio).max(1.).trunc();

        let center = lookfrom;

        // Determine viewport dimensions.
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focus_dist;
        let viewport_width = viewport_height * (image_width / image_height);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - focus_dist * w - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vector.
        let defocus_radius = focus_dist * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            center,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            image_height: image_height as _,
            image_width: image_width as _,
            max_depth,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {:05}", self.image_height - j);
            for i in 0..self.image_width {
                let i: f64 = i.try_into().unwrap();
                let j: f64 = j.try_into().unwrap();

                let mut pixel_color = Color3::new(0., 0., 0.);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                pixel_color.write_color(self.samples_per_pixel);
            }
        }
        eprintln!("\rDone.{}", " ".repeat(25));
    }

    fn ray_color(r: &Ray, depth: u32, world: &dyn Hittable) -> Color3 {
        if depth == 0 {
            // If we've exceeded the ray bounce limit, no more light is gathered.
            Color3::new(0., 0., 0.)
        } else if let Some(rec) = world.hit(r, &Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
                attenuation * Self::ray_color(&scattered, depth - 1, world)
            } else {
                Color3::new(0., 0., 0.)
            }
        } else {
            let unit_direction = r.direction.unit_vector();
            let a = 0.5 * (unit_direction.y + 1.);
            (1. - a) * WHITE + a * LIGHT_BLUE
        }
    }

    /// Get a randomly-sampled camera ray for the pixel at location i,j, originating
    /// from the camera defocus disk.
    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let pixel_center = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = match self.defocus_angle <= 0. {
            true => self.center,
            false => self.defocus_disk_sample(),
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// Returns a random point in the square surrouding a pixel at the origin.
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}
