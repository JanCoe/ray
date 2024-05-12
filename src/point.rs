use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64, pub f64);

impl Point {
    pub fn dot(self, rhs: Point) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn roll(self, gamma: f64) -> Self {
        // counter-clockwise about the x-axis
        let gamma = gamma.to_radians();
        Self(
            self.0,
            self.1 * gamma.cos() - self.2 * gamma.sin(),
            self.1 * gamma.sin() + self.2 * gamma.cos(),
        )
    }
    pub fn pitch(self, beta: f64) -> Self {
        // counter-clockwise about the y-axis
        let beta = beta.to_radians();
        Self(
            self.0 * beta.cos() + self.2 * beta.sin(),
            self.1,
            -self.0 * beta.sin() + self.2 * beta.cos(),
        )
    }
    pub fn yaw(self, alpha: f64) -> Self {
        let alpha = alpha.to_radians();
        // counter-clockwise about the z-axis
        Self(
            self.0 * alpha.cos() - self.1 * alpha.sin(),
            self.0 * alpha.sin() + self.1 * alpha.cos(),
            self.2,
        )
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self {
        Point(-self.0, -self.1, -self.2)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Point(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}
