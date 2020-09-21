use crate::pixel::Pixel;

pub trait Buffer<P: Pixel> {
    fn dimensions(&self) -> (u32, u32);

    fn get_pixel(&self, x: u32, y: u32) -> &P;

    fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut P;

    fn put_pixel(&mut self, x: u32, y: u32, pixel: P);
}
