use std::convert::Into;
use std::ops::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,

}
pub type Point2 = Vector2;

#[derive(Default)]
pub enum AllowZero {
    TRUE,
    #[default]
    FALSE,
}

#[derive(Default)]
pub enum Polarity {
    #[default]
    TRUE,
    FALSE,
}


// Static Methods
impl Vector2 {
    pub fn new_empty() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn new_splat(val: f64) -> Self {
        Self { x: val, y: val }
    }

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

// Instance Methods
impl Vector2 {
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    /// Default to not allow zero length
    pub fn normalize(&mut self, allow_zero: AllowZero) -> Self {
        let mut x: f64 = self.x;
        let mut y: f64 = self.y;
        if self.length() > 0. {
            let len = self.length();
            x /= len;
            y /= len;
        } else {
            match allow_zero {
                AllowZero::TRUE => {
                    x = 0.;
                    y = 0.;
                }
                AllowZero::FALSE => {
                    x = 0.;
                    y = 1.;
                }
            }
        }

        Vector2::new(x, y)
    }

    /// Default to polarity=true
    pub fn get_orthogonal(&self, polarity: Polarity) -> Self {
        match polarity {
            Polarity::TRUE => Vector2::new(-self.y, self.x),
            Polarity::FALSE => Vector2::new(self.y, -self.x),
        }
    }

    /// Default to polarity=true, allow_zero=false
    pub fn getOrthonormal(&self, polarity: Polarity, allow_zero: Polarity) -> Self {
        match polarity {
            Polarity::TRUE => {
                Vector2::new(-self.y, self.x).normalize(AllowZero::default())
            },
            Polarity::FALSE => {
                Vector2::new(self.y, -self.x).normalize(AllowZero::default())
            },
        }
    }

}

impl Into<bool> for Vector2 {
    fn into(self) -> bool {
        self.x != 0.0 || self.y != 0.0
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vector2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Div<f64> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f64> for Vector2 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
    }
}

impl Div<Vector2> for f64 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self / rhs.x, self / rhs.y)
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Vector2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self {
        Vector2::new(-self.x, -self.y)
    }
}

pub fn dot_product(a: Vector2, b: Vector2) -> f64 {
    a.x * b.x + a.y * b.y
}

pub fn cross_product(a: Vector2, b: Vector2) -> f64 {
    a.x * b.y - a.y * b.x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let c = a + b;
        assert_eq!(c, Vector2::new(4.0, 6.0));
        
        a += b;
        assert_eq!(c, a);
    }

    #[test]
    fn subtraction() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let c = a - b;
        assert_eq!(c, Vector2::new(-2.0, -2.0));
        
        a -= b;
        assert_eq!(c, a);
    }

    #[test]
    fn vec_multiplication() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = 2.0;
        let c = a * b;
        assert_eq!(c, Vector2::new(2.0, 4.0));
        
        a *= b;
        assert_eq!(c, a);
    }

    #[test]
    fn sclr_multiplication() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = 2.0;
        let c = a * b;
        assert_eq!(c, Vector2::new(2.0, 4.0));
        
        a *= b;
        assert_eq!(c, a);
    }

    #[test]
    fn vec_division() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let c = a / b;
        assert_eq!(c, Vector2::new(1.0/3.0, 2.0/4.0));
        
        a /= b;
        assert_eq!(c, a);
    }

    #[test]
    fn sclr_division() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = 2.0;
        let c = a / b;
        assert_eq!(c, Vector2::new(1.0/2.0, 2.0/2.0));
        
        let d = b / a;
        assert_eq!(d, Vector2::new(2.0/1.0, 2.0/2.0));

        a /= b;
        assert_eq!(c, a);
    }
}
