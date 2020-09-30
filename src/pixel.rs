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
