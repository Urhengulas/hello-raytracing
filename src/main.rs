mod vec3;

use crate::vec3::Color;

fn main() {
    // Image

    let image_width = 1024;
    let image_height = 1024;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {:05}", image_height - j);
        for i in 0..image_width {
            let i: f64 = i.try_into().unwrap();
            let j: f64 = j.try_into().unwrap();
            let image_width: f64 = image_width.try_into().unwrap();
            let image_height: f64 = image_height.try_into().unwrap();

            let r = i / (image_width - 1.0);
            let g = j / (image_height - 1.0);
            let b = 0.0;

            let pixel_color = Color::new(r, g, b);
            pixel_color.write_color();
        }
    }
    eprintln!("\rDone.{}", " ".repeat(25));
}
