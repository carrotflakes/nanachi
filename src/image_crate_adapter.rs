//! Interfaces for [image crate](https://github.com/image-rs/image).

use crate::buffer::{Buffer, GenericBuffer};
use crate::pixel::Pixel;

impl Pixel for image::Rgb<u8> {
    fn lerp(&self, rhs: &Self, rate: f64) -> Self {
        use image::Pixel;
        self.clone().map2(&rhs, |a, b| {
            (a as f64 * (1.0 - rate) + b as f64 * rate).round() as u8
        })
    }
}
impl Pixel for image::Rgba<u8> {
    fn lerp(&self, rhs: &Self, rate: f64) -> Self {
        use image::Pixel;
        self.clone().map2(&rhs, |a, b| {
            (a as f64 * (1.0 - rate) + b as f64 * rate).round() as u8
        })
    }
}

impl<S: image::Primitive + 'static, P: Pixel + image::Pixel<Subpixel = S> + 'static> Buffer<P>
    for image::ImageBuffer<P, Vec<S>>
{
    fn dimensions(&self) -> (u32, u32) {
        self.dimensions()
    }

    fn get_pixel(&self, x: u32, y: u32) -> &P {
        self.get_pixel(x, y)
    }

    fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut P {
        <Self as image::GenericImage>::get_pixel_mut(self, x, y)
    }

    fn put_pixel(&mut self, x: u32, y: u32, pixel: P) {
        <Self as image::GenericImage>::put_pixel(self, x, y, pixel)
    }
}

impl Into<image::Rgba<u8>> for crate::pixel::Rgba {
    #[inline]
    fn into(self) -> image::Rgba<u8> {
        image::Rgba([
            (self.0[0].clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.0[1].clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.0[2].clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.0[3].clamp(0.0, 1.0) * 255.0).round() as u8,
        ])
    }
}

impl Into<image::Rgba<u8>> for crate::pixel::PremultipliedRgba {
    #[inline]
    fn into(self) -> image::Rgba<u8> {
        image::Rgba([
            ((self.0[0] / self.0[3]).clamp(0.0, 1.0) * 255.0).round() as u8,
            ((self.0[1] / self.0[3]).clamp(0.0, 1.0) * 255.0).round() as u8,
            ((self.0[2] / self.0[3]).clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.0[3].clamp(0.0, 1.0) * 255.0).round() as u8,
        ])
    }
}

impl<P: Pixel + Into<image::Rgba<u8>>> Into<image::RgbaImage> for &GenericBuffer<P> {
    fn into(self) -> image::RgbaImage {
        image::RgbaImage::from_fn(self.dimensions().0, self.dimensions().1, |x, y| {
            self.get_pixel(x, y).clone().into()
        })
    }
}
