use crate::buffer::Buffer;
use crate::fill_color::FillColor;
use crate::interpolation::Interpolation;
use crate::pixel::Pixel;

/// Tiling an image.
#[derive(Debug, Clone)]
pub struct Pattern<'a, P: Pixel, B: Buffer<P>, I: Interpolation<P, B>> {
    width: f64,
    height: f64,
    image: &'a B,
    interpolation: I,
    pixel: std::marker::PhantomData<P>,
}

impl<'a, P: Pixel, B: Buffer<P>, I: Interpolation<P, B>> Pattern<'a, P, B, I> {
    pub fn new(image: &'a B, interpolation: I) -> Self {
        let (width, height) = image.dimensions();
        Pattern {
            width: width as f64,
            height: height as f64,
            image,
            interpolation,
            pixel: Default::default(),
        }
    }
}

impl<'a, P: Pixel, B: Buffer<P>, I: Interpolation<P, B>> FillColor<P> for Pattern<'a, P, B, I> {
    fn fill_color(&self, x: f64, y: f64) -> P {
        self.interpolation.interpolate(&self.image, x, y)
    }
}
