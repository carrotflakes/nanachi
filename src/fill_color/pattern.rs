use crate::buffer::Buffer;
use crate::fill_color::FillColor;
use crate::interpolation::Interpolation;
use crate::pixel::Pixel;

/// Tiling an image.
#[derive(Debug, Clone)]
pub struct Pattern<P, B, DB, I>
where
    P: Pixel,
    B: Buffer<P>,
    DB: std::ops::Deref<Target = B>,
    I: Interpolation<P, B>,
{
    image: DB,
    interpolation: I,
    _pixel: std::marker::PhantomData<P>,
}

impl<P, B, DB, I> Pattern<P, B, DB, I>
where
    P: Pixel,
    B: Buffer<P>,
    DB: std::ops::Deref<Target = B>,
    I: Interpolation<P, B>,
{
    pub fn new(image: DB, interpolation: I) -> Self {
        Pattern {
            image,
            interpolation,
            _pixel: Default::default(),
        }
    }
}

impl<P, B, DB, I> FillColor<P> for Pattern<P, B, DB, I>
where
    P: Pixel,
    B: Buffer<P>,
    DB: std::ops::Deref<Target = B>,
    I: Interpolation<P, B>,
{
    fn fill_color(&self, x: f64, y: f64) -> P {
        self.interpolation.interpolate(&self.image, x, y)
    }
}
