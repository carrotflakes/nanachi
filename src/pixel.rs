pub trait Pixel: Clone + 'static {
    fn lerp(&self, rhs: &Self, rate: f64) -> Self;
}

#[derive(Clone)]
pub struct RgbaF32(pub [f32; 4]);

impl Pixel for RgbaF32 {
    fn lerp(&self, rhs: &Self, rate: f64) -> Self {
        let r = rate as f32;
        let i = 1.0 - r;
        RgbaF32([
            self.0[0] * i + rhs.0[0] * r,
            self.0[1] * i + rhs.0[1] * r,
            self.0[2] * i + rhs.0[2] * r,
            self.0[3] * i + rhs.0[3] * r,
        ])
    }
}
