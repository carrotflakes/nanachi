use super::*;

/// Premultiplied RGBA color
///
/// [`PremultipliedRgba`] keeps RGB value that premultiplied alpha.
/// It may be faster than [`Rgba`] but inaccurate on transparent pixels.
#[derive(Clone, Copy)]
pub struct PremultipliedRgba(pub [f32; 4]);

impl Pixel for PremultipliedRgba {
    fn lerp(&self, rhs: &Self, rate: f64) -> Self {
        let i = 1.0 - rate as f32;
        PremultipliedRgba([
            self.0[0] * i + rhs.0[0] * rate as f32,
            self.0[1] * i + rhs.0[1] * rate as f32,
            self.0[2] * i + rhs.0[2] * rate as f32,
            self.0[3] * i + rhs.0[3] * rate as f32,
        ])
    }
}

impl Add for PremultipliedRgba {
    type Output = Self;

    fn add(self, rhs: PremultipliedRgba) -> Self::Output {
        PremultipliedRgba([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1], self.0[2] + rhs.0[2], self.0[3] + rhs.0[3]])
    }
}

impl Mul for PremultipliedRgba {
    type Output = Self;

    fn mul(self, rhs: PremultipliedRgba) -> Self::Output {
        PremultipliedRgba([self.0[0] * rhs.0[0], self.0[1] * rhs.0[1], self.0[2] * rhs.0[2], self.0[3] * rhs.0[3]])
    }
}

impl Mul<f32> for PremultipliedRgba {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        PremultipliedRgba([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs, self.0[3] * rhs])
    }
}

impl Arithmetic for PremultipliedRgba {
    fn zero() -> Self {
        PremultipliedRgba([0.0, 0.0, 0.0, 0.0])
    }
}
