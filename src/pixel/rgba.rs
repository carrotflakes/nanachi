use super::*;

/// RGBA color
#[derive(Clone, Copy)]
pub struct Rgba(pub [f32; 4]);

impl Pixel for Rgba {
    fn lerp(&self, rhs: &Self, rate: f32) -> Self {
        Rgba([
            self.0[0] + (rhs.0[0] - self.0[0]) * rate,
            self.0[1] + (rhs.0[1] - self.0[1]) * rate,
            self.0[2] + (rhs.0[2] - self.0[2]) * rate,
            self.0[3] + (rhs.0[3] - self.0[3]) * rate,
        ])
    }
}

impl Add for Rgba {
    type Output = Self;

    fn add(self, rhs: Rgba) -> Self::Output {
        Rgba([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl Sub for Rgba {
    type Output = Self;

    fn sub(self, rhs: Rgba) -> Self::Output {
        Rgba([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl Mul for Rgba {
    type Output = Self;

    fn mul(self, rhs: Rgba) -> Self::Output {
        Rgba([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
            self.0[3] * rhs.0[3],
        ])
    }
}

impl Mul<f32> for Rgba {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Rgba([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
        ])
    }
}

impl Arithmetic for Rgba {
    fn zero() -> Self {
        Rgba([0.0, 0.0, 0.0, 0.0])
    }
}
