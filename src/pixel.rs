/// A trait for represents a pixel.
pub trait Pixel: Clone + 'static {
    fn lerp(&self, rhs: &Self, rate: f64) -> Self;
}

/// RGBA color
#[derive(Clone, Copy)]
pub struct Rgba(pub [f32; 4]);

impl Pixel for Rgba {
    fn lerp(&self, rhs: &Self, rate: f64) -> Self {
        let i = 1.0 - rate as f32;
        Rgba([
            self.0[0] * i + rhs.0[0] * rate as f32,
            self.0[1] * i + rhs.0[1] * rate as f32,
            self.0[2] * i + rhs.0[2] * rate as f32,
            self.0[3] * i + rhs.0[3] * rate as f32,
        ])
    }
}

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
