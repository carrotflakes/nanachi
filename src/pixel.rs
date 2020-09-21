pub trait Pixel: Clone + 'static {
    fn lerp(&self, rhs: &Self, rate: f64) -> Self;
}
