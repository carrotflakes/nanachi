use crate::fill_color::FillColor;
use crate::pixel::Pixel;
use image::ImageBuffer;

#[derive(Debug, Clone)]
pub struct Pattern<'a, P: Pixel> {
    width: f64,
    height: f64,
    image: &'a ImageBuffer<P, Vec<u8>>,
}

impl<'a, P: Pixel> Pattern<'a, P> {
    pub fn new(image: &'a ImageBuffer<P, Vec<u8>>) -> Self {
        Pattern {
            width: image.width() as f64,
            height: image.height() as f64,
            image,
        }
    }
}

impl<'a, P: Pixel> FillColor<P> for Pattern<'a, P> {
    fn fill_color(&self, x: f64, y: f64) -> P {
        *self.image.get_pixel(
            x.rem_euclid(self.width) as u32,
            y.rem_euclid(self.height) as u32,
        )
    }
}
