fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

fn random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}
