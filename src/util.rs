pub fn _degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

/// Returns a random real in [0,1).
pub fn random_double() -> f64 {
    rand::random::<f64>()
}

/// Returns a random real in [min,max).
pub fn _random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}
