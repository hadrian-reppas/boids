use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

use crate::vertex::Vertex;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn zero() -> Self {
        Vector::new(0.0, 0.0)
    }

    pub fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }

    pub fn normalize(self) -> Self {
        let denom = 1.0 / self.x.hypot(self.y);
        Vector::new(self.x * denom, self.y * denom)
    }

    pub fn orthogonal(self) -> Self {
        Vector::new(-self.y, self.x)
    }

    pub fn map(self, width: f32, height: f32) -> Vertex {
        Vertex {
            x: 2.0 * self.x / width - 1.0,
            y: -2.0 * self.y / height + 1.0,
        }
    }

    pub fn from_angle(theta: f32) -> Self {
        Vector {
            x: theta.cos(),
            y: theta.sin(),
        }
    }

    pub fn magnitude(self) -> f32 {
        self.x.hypot(self.y)
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y)
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}
