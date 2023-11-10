use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{
    interval::Interval,
    util::{linear_to_gamma, random_double_minmax},
};

pub type Color3 = Vec3;
pub type Point3 = Vec3;

pub const LIGHT_BLUE: Color3 = Color3::new(0.5, 0.7, 1.);
pub const WHITE: Color3 = Color3::new(1., 1., 1.);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn _cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    /// Return true of the vector is close to zero in all dimenstions.
    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    fn random(min: f64, max: f64) -> Self {
        Vec3::new(
            random_double_minmax(min, max),
            random_double_minmax(min, max),
            random_double_minmax(min, max),
        )
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    fn _random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if normal.dot(&on_unit_sphere) > 0. {
            // In the same hemisphere as the normal
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn write_color(&self, samples_per_pixel: u32) {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        // Divide the color by the number of samples.
        let samples_per_pixel = Into::<f64>::into(samples_per_pixel);
        let scale = 1. / samples_per_pixel;
        r *= scale;
        g *= scale;
        b *= scale;

        // Apply the linear to gamma transform
        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        // Write the translated [0,255] value of each color component.
        const INTENSITY: Interval = Interval::new(0., 0.999);
        println!(
            "{} {} {}",
            (256. * INTENSITY.clamp(r)) as i64,
            (256. * INTENSITY.clamp(g)) as i64,
            (256. * INTENSITY.clamp(b)) as i64,
        );
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, mut rhs: Vec3) -> Self::Output {
        rhs *= self;
        rhs
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross() {
        // Arrange
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);

        // Act
        let c = a._cross(&b);

        // Assert
        let result = Vec3::new(-3., 6., -3.);
        assert_eq!(c, result);
    }

    #[test]
    fn dot() {
        // Arrange
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);

        // Act
        let c = a.dot(&b);

        // Assert
        let result = 32.;
        assert_eq!(c, result);
    }

    #[test]
    fn length() {
        // Arrange
        let a = Vec3::new(3., 4., 0.);

        // Act
        let b = a.length();

        // Assert
        let result = 5.;
        assert_eq!(b, result);
    }

    #[test]
    fn unit_vector() {
        // Arrange
        let a = Vec3::new(3., 4., 0.);

        // Act
        let b = a.unit_vector();

        // Assert
        let result = Vec3::new(0.6, 0.8, 0.);
        assert_eq!(b, result);
    }

    #[test]
    fn add() {
        // Arrange
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);

        // Act
        let c = a + b;

        // Assert
        let result = Vec3::new(5., 7., 9.);
        assert_eq!(c, result);
    }

    #[test]
    fn add_f64() {
        // Arrange
        let a = Vec3::new(1., 2., 3.);
        let b = 2.;

        // Act
        let c = a + b;

        // Assert
        let result = Vec3::new(3., 4., 5.);
        assert_eq!(c, result);
    }

    #[test]
    fn add_assign() {
        // Arrange
        let mut a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);

        // Act
        a += b;

        // Assert
        let result = Vec3::new(5., 7., 9.);
        assert_eq!(a, result);
    }

    #[test]
    fn div() {
        // Arrange
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);

        // Act
        let c = a / b;

        // Assert
        let result = Vec3::new(0.25, 0.4, 0.5);
        assert_eq!(c, result);
    }

    #[test]
    fn div_f64() {
        // Arrange
        let a = Vec3::new(1., 2., 3.);
        let b = 2.;

        // Act
        let c = a / b;

        // Assert
        let result = Vec3::new(0.5, 1., 1.5);
        assert_eq!(c, result);
    }

    #[test]
    fn div_assign() {
        // Arrange
        let mut a = Vec3::new(1., 2., 3.);
        let b = 2.;

        // Act
        a /= b;

        // Assert
        let result = Vec3::new(0.5, 1., 1.5);
        assert_eq!(a, result);
    }

    #[test]
    fn mul() {
        // Arrange
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);

        // Act
        let c = a * b;

        // Assert
        let result = Vec3::new(4., 10., 18.);
        assert_eq!(c, result);
    }

    #[test]
    fn mul_f64() {
        // Arrange
        let a = Vec3::new(1., 2., 3.);
        let b = 2.;

        // Act
        let c = a * b;

        // Assert
        let result = Vec3::new(2., 4., 6.);
        assert_eq!(c, result);
    }

    #[test]
    fn mul_vec3() {
        // Arrange
        let a = Vec3::new(1., 2., 3.);
        let b = 2.;

        // Act
        let c = b * a;

        // Assert
        let result = Vec3::new(2., 4., 6.);
        assert_eq!(c, result);
    }

    #[test]
    fn mul_assign() {
        // Arrange
        let mut a = Vec3::new(1., 2., 3.);
        let b = 2.;

        // Act
        a *= b;

        // Assert
        let result = Vec3::new(2., 4., 6.);
        assert_eq!(a, result);
    }

    #[test]
    fn neg() {
        // Arrange
        let a = Vec3::new(1., -2., 3.);

        // Act
        let b = -a;

        // Assert
        let result = Vec3::new(-1., 2., -3.);
        assert_eq!(b, result);
    }

    #[test]
    fn sub() {
        // Arrange
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);

        // Act
        let c = a - b;

        // Assert
        let result = Vec3::new(-3., -3., -3.);
        assert_eq!(c, result);
    }

    #[test]
    fn sub_assign() {
        // Arrange
        let mut a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);

        // Act
        a -= b;

        // Assert
        let result = Vec3::new(-3., -3., -3.);
        assert_eq!(a, result);
    }
}
