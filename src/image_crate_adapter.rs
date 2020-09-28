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

impl Into<image::RgbaImage> for &GenericBuffer<crate::pixel::Rgba> {
    fn into(self) -> image::RgbaImage {
        image::RgbaImage::from_fn(self.dimensions().0, self.dimensions().1, |x, y| {
            let p = self.get_pixel(x, y);
            image::Rgba([
                (p.0[0].min(1.0).max(0.0) * 255.0).round() as u8,
                (p.0[1].min(1.0).max(0.0) * 255.0).round() as u8,
                (p.0[2].min(1.0).max(0.0) * 255.0).round() as u8,
                (p.0[3].min(1.0).max(0.0) * 255.0).round() as u8,
            ])
        })
    }
}
