//! [`Pixel`] trait represents a pixel.

mod premultiplied_rgba;
mod rgba;

use std::ops::{Add, Mul, Sub};

pub use premultiplied_rgba::PremultipliedRgba;
pub use rgba::Rgba;

/// A trait for represents a pixel.
pub trait Pixel: Clone + 'static {
    fn lerp(&self, rhs: &Self, rate: f64) -> Self;
}

pub trait Arithmetic:
    Sized + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Mul<f32, Output = Self>
{
    fn zero() -> Self;
}

impl Into<Rgba> for PremultipliedRgba {
    fn into(self) -> Rgba {
        let [r, g, b, a] = self.0;
        if a == 0.0 {
            Rgba([r / a, g / a, b / a, a])
        } else {
            Rgba([0.0, 0.0, 0.0, 0.0])
        }
    }
}

impl Into<PremultipliedRgba> for Rgba {
    fn into(self) -> PremultipliedRgba {
        let [r, g, b, a] = self.0;
        PremultipliedRgba([r * a, g * a, b * a, a])
    }
}

impl Pixel for f64 {
    fn lerp(&self, rhs: &Self, rate: f64) -> Self {
        self * (1.0 - rate) + rhs * rate
    }
}
