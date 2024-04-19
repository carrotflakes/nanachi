//! Collection of interpolation types.
//!
//! Currently, 2 types of interpolation are available.
//! - NearestNeighbor
//! - Bilinear

use crate::buffer::Buffer;
use crate::pixel::Pixel;
use crate::point::Point;

pub trait Interpolation<P: Pixel, B: Buffer<P>> {
    fn interpolate(&self, buffer: &B, pos: Point) -> P;
}

/// Nearest neighbor interpolation.
#[derive(Clone)]
pub struct NearestNeighbor;

impl<P: Pixel, B: Buffer<P>> Interpolation<P, B> for NearestNeighbor {
    fn interpolate(&self, buffer: &B, pos: Point) -> P {
        let (width, height) = buffer.dimensions();
        buffer
            .get_pixel(
                (pos.x().rem_euclid(width as f32).floor() as u32).min(width - 1),
                (pos.y().rem_euclid(height as f32).floor() as u32).min(height - 1),
            )
            .clone()
    }
}

/// Bi-linear interpolation.
#[derive(Clone)]
pub struct Bilinear;

impl<P: Pixel, B: Buffer<P>> Interpolation<P, B> for Bilinear {
    fn interpolate(&self, buffer: &B, pos: Point) -> P {
        let (width, height) = buffer.dimensions();
        let mut x = pos.x().rem_euclid(width as f32);
        let mut y = pos.y().rem_euclid(height as f32);
        let x2 = (x as u32 + 1) % width;
        let y2 = (y as u32 + 1) % height;
        if x as u32 == width {
            x = width as f32 - 0.01;
        }
        if y as u32 == height {
            y = height as f32 - 0.01;
        }
        buffer
            .get_pixel(x as u32, y as u32)
            .lerp(buffer.get_pixel(x2, y as u32), x.fract())
            .lerp(
                &buffer
                    .get_pixel(x as u32, y2)
                    .lerp(buffer.get_pixel(x2, y2), x.fract()),
                y.fract(),
            )
    }
}
