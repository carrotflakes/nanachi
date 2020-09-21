use crate::buffer::Buffer;
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

impl<P: Pixel + image::Pixel<Subpixel = u8> + 'static> Buffer<P>
    for image::ImageBuffer<P, Vec<u8>>
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
