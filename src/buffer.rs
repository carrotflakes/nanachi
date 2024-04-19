//! [`Buffer`] trait represents an image.

use crate::pixel::Pixel;

/// A trait for represents an image.
pub trait Buffer<P: Pixel> {
    /// Get `(width, height)`.
    fn dimensions(&self) -> (u32, u32);

    /// Get pixel by `x` and `y`.
    fn get_pixel(&self, x: u32, y: u32) -> &P;

    /// Put a pixel.
    fn put_pixel(&mut self, x: u32, y: u32, pixel: P);
}

/// 2D image buffer for manipulation.
#[derive(Clone)]
pub struct GenericBuffer<P: Pixel> {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) buffer: Vec<P>,
}

impl<P: Pixel> Buffer<P> for GenericBuffer<P> {
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn get_pixel(&self, x: u32, y: u32) -> &P {
        &self.buffer[(y * self.width + x) as usize]
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

    pub fn as_slice(&self) -> &[P] {
        &self.buffer
    }
}
