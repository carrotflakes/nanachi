use crate::buffer::Buffer;
use crate::fill_color::FillColor;
use crate::interpolation::Interpolation;
use crate::matrix::Matrix;
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
    /// Expects inverted matrix.
    matrix: Matrix,
    _pixel: std::marker::PhantomData<P>,
}

impl<P, B, DB, I> Pattern<P, B, DB, I>
where
    P: Pixel,
    B: Buffer<P>,
    DB: std::ops::Deref<Target = B>,
    I: Interpolation<P, B>,
{
    pub fn new(image: DB, interpolation: I, matrix: Matrix) -> Self {
        Pattern {
            image,
            interpolation,
            matrix,
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
    fn fill_color(&self, x: f32, y: f32) -> P {
        let (x, y) = self.matrix.apply((x, y));
        self.interpolation.interpolate(&self.image, x, y)
    }
}
