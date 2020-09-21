use crate::buffer::Buffer;
use crate::fill_color::FillColor;
use crate::pixel::Pixel;

#[derive(Debug, Clone)]
pub struct Pattern<'a, P: Pixel, B: Buffer<P>> {
    width: f64,
    height: f64,
    image: &'a B,
    pixel: std::marker::PhantomData<P>,
}

impl<'a, P: Pixel, B: Buffer<P>> Pattern<'a, P, B> {
    pub fn new(image: &'a B) -> Self {
        let (width, height) = image.dimensions();
        Pattern {
            width: width as f64,
            height: height as f64,
            image,
            pixel: Default::default(),
        }
    }
}

impl<'a, P: Pixel, B: Buffer<P>> FillColor<P> for Pattern<'a, P, B> {
    fn fill_color(&self, x: f64, y: f64) -> P {
        self.image
            .get_pixel(
                x.rem_euclid(self.width) as u32,
                y.rem_euclid(self.height) as u32,
            )
            .clone()
    }
}
