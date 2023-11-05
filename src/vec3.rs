use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Color = Vec3;
pub type Point3 = Vec3;

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

    fn cross(&self, rhs: &Self) -> Self {
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
        self.dot(self).sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn write_color(&self) {
        let a = |b| (255.999 * b) as i64;
        println!("{} {} {}", a(self.x), a(self.y), a(self.z));
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
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
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        // Act
        let c = a.cross(&b);

        // Assert
        let result = Vec3::new(-3.0, 6.0, -3.0);
        assert_eq!(c, result);
    }

    #[test]
    fn dot() {
        // Arrange
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        // Act
        let c = a.dot(&b);

        // Assert
        let result = 32.0;
        assert_eq!(c, result);
    }

    #[test]
    fn length() {
        // Arrange
        let a = Vec3::new(3.0, 4.0, 0.0);

        // Act
        let b = a.length();

        // Assert
        let result = 5.0;
        assert_eq!(b, result);
    }

    #[test]
    fn unit_vector() {
        // Arrange
        let a = Vec3::new(3.0, 4.0, 0.0);

        // Act
        let b = a.unit_vector();

        // Assert
        let result = Vec3::new(0.6, 0.8, 0.0);
        assert_eq!(b, result);
    }

    #[test]
    fn add() {
        // Arrange
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        // Act
        let c = a + b;

        // Assert
        let result = Vec3::new(5.0, 7.0, 9.0);
        assert_eq!(c, result);
    }

    #[test]
    fn add_assign() {
        // Arrange
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        // Act
        a += b;

        // Assert
        let result = Vec3::new(5.0, 7.0, 9.0);
        assert_eq!(a, result);
    }

    #[test]
    fn div() {
        // Arrange
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        // Act
        let c = a / b;

        // Assert
        let result = Vec3::new(0.25, 0.4, 0.5);
        assert_eq!(c, result);
    }

    #[test]
    fn div_f64() {
        // Arrange
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;

        // Act
        let c = a / b;

        // Assert
        let result = Vec3::new(0.5, 1.0, 1.5);
        assert_eq!(c, result);
    }

    #[test]
    fn div_assign() {
        // Arrange
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;

        // Act
        a /= b;

        // Assert
        let result = Vec3::new(0.5, 1.0, 1.5);
        assert_eq!(a, result);
    }

    #[test]
    fn mul() {
        // Arrange
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        // Act
        let c = a * b;

        // Assert
        let result = Vec3::new(4.0, 10.0, 18.0);
        assert_eq!(c, result);
    }

    #[test]
    fn mul_f64() {
        // Arrange
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;

        // Act
        let c = a * b;

        // Assert
        let result = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(c, result);
    }

    #[test]
    fn mul_vec3() {
        // Arrange
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;

        // Act
        let c = b * a;

        // Assert
        let result = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(c, result);
    }

    #[test]
    fn mul_assign() {
        // Arrange
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;

        // Act
        a *= b;

        // Assert
        let result = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(a, result);
    }

    #[test]
    fn neg() {
        // Arrange
        let a = Vec3::new(1.0, -2.0, 3.0);

        // Act
        let b = -a;

        // Assert
        let result = Vec3::new(-1.0, 2.0, -3.0);
        assert_eq!(b, result);
    }

    #[test]
    fn sub() {
        // Arrange
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        // Act
        let c = a - b;

        // Assert
        let result = Vec3::new(-3.0, -3.0, -3.0);
        assert_eq!(c, result);
    }

    #[test]
    fn sub_assign() {
        // Arrange
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        // Act
        a -= b;

        // Assert
        let result = Vec3::new(-3.0, -3.0, -3.0);
        assert_eq!(a, result);
    }
}
