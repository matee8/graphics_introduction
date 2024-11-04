use core::ops::{Add, Mul, Sub};

use crate::Point;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    #[inline]
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl From<Vector2> for Point {
    #[inline]
    fn from(value: Vector2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Point> for Vector2 {
    #[inline]
    fn from(value: Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Add for Vector2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector2> for f64 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Vector2) -> Self::Output {
        rhs * self
    }
}