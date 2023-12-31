pub fn f64(a: i32) -> f64 {
    a.into()
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

/// Returns a random real in [0,1).
pub fn random_double() -> f64 {
    rand::random::<f64>()
}

/// Returns a random real in [min,max).
pub fn random_double_minmax(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
