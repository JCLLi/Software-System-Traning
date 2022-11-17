use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct TextureCoordinate {
    pub u: f64,
    pub v: f64,
}

impl TextureCoordinate {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }
}

impl Add for TextureCoordinate {
    type Output = TextureCoordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            u: self.u + rhs.u,
            v: self.v + rhs.v,
        }
    }
}

impl Sub for TextureCoordinate {
    type Output = TextureCoordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            u: self.u - rhs.u,
            v: self.v - rhs.v,
        }
    }
}

impl Sub for &TextureCoordinate {
    type Output = TextureCoordinate;

    fn sub(self, rhs: &TextureCoordinate) -> Self::Output {
        TextureCoordinate {
            u: self.u - rhs.u,
            v: self.v - rhs.v,
        }
    }
}

impl Mul for TextureCoordinate {
    type Output = TextureCoordinate;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            u: self.u * rhs.u,
            v: self.v * rhs.v,
        }
    }
}

impl Mul<f64> for TextureCoordinate {
    type Output = TextureCoordinate;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            u: self.u * rhs,
            v: self.v * rhs,
        }
    }
}
