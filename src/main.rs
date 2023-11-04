fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        for i in 0..image_width {
            let i: f64 = i.try_into().unwrap();
            let j: f64 = j.try_into().unwrap();
            let image_width: f64 = image_width.try_into().unwrap();
            let image_height: f64 = image_height.try_into().unwrap();

            let r = i / (image_width - 1.0);
            let g = j / (image_height - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            println!("{ir} {ig} {ib}");
        }
    }
}
