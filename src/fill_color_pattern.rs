use crate::fill_color::FillColor;
use image::{ImageBuffer, Pixel};

#[derive(Debug, Clone)]
pub struct Pattern<'a, C: Pixel<Subpixel = u8> + 'static> {
    width: f64,
    height: f64,
    image: &'a ImageBuffer<C, Vec<u8>>,
}

impl<'a, C: Pixel<Subpixel = u8> + 'static> Pattern<'a, C> {
    pub fn new(image: &'a ImageBuffer<C, Vec<u8>>) -> Self {
        Pattern {
            width: image.width() as f64,
            height: image.height() as f64,
            image,
        }
    }
}

impl<'a, C: Pixel<Subpixel = u8> + 'static> FillColor<C> for Pattern<'a, C> {
    fn fill_color(&self, x: f64, y: f64) -> C {
        *self.image.get_pixel(
            x.rem_euclid(self.width) as u32,
            y.rem_euclid(self.height) as u32,
        )
    }
}
