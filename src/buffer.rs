use crate::pixel::Pixel;

pub trait Buffer<P: Pixel> {
    fn dimensions(&self) -> (u32, u32);

    fn get_pixel(&self, x: u32, y: u32) -> &P;

    fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut P;

    fn put_pixel(&mut self, x: u32, y: u32, pixel: P);
}

pub struct GenericBuffer<P: Pixel> {
    width: u32,
    height: u32,
    buffer: Vec<P>,
}

impl<P: Pixel> Buffer<P> for GenericBuffer<P> {
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn get_pixel(&self, x: u32, y: u32) -> &P {
        &self.buffer[(y * self.width + x) as usize]
    }

    fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut P {
        &mut self.buffer[(y * self.width + x) as usize]
    }

    fn put_pixel(&mut self, x: u32, y: u32, pixel: P) {
        self.buffer[(y * self.width + x) as usize] = pixel;
    }
}

impl<P: Pixel> GenericBuffer<P> {
    pub fn from_pixel(width: u32, height: u32, pixel: P) -> Self {
        GenericBuffer {
            width,
            height,
            buffer: vec![pixel; (width * height) as usize],
        }
    }
}
