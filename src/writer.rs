use crate::position_color::PositionColor;
use image::{ImageBuffer, Pixel};

pub fn alpha_blend<'a, X, C: PositionColor<X>>(
    buf: &'a mut ImageBuffer<X, Vec<u8>>,
    position_color: &'a C,
) -> impl FnMut(u32, u32, f64) + 'a
where
    X: Pixel<Subpixel = u8> + 'static {
    move |x: u32, y: u32, v: f64|
    img_blend_pixel(buf, position_color, x, y, v)
}

pub fn alpha_blend2<'a, X, C: PositionColor<X>>(
    buf: &'a mut ImageBuffer<X, Vec<u8>>,
    position_color: &'a C,
    alpha: f64,
) -> impl FnMut(u32, u32, f64) + 'a
where
    X: Pixel<Subpixel = u8> + 'static {
    move |x: u32, y: u32, v: f64|
    img_blend_pixel(buf, position_color, x, y, v * alpha)
}

pub fn img_blend_pixel<X, C: PositionColor<X>>(
    buf: &mut ImageBuffer<X, Vec<u8>>,
    position_color: &C,
    x: u32,
    y: u32,
    r: f64,
) where
    X: Pixel<Subpixel = u8> + 'static,
{
    if x < buf.width() && y < buf.height() {
        let pixel = position_color.position_color((x, y).into());
        let pixel = blend_pixel(*buf.get_pixel(x, y), pixel, r);
        buf.put_pixel(x, y, pixel);
    }
}

pub fn blend_pixel<X>(p1: X, p2: X, r: f64) -> X
where
    X: Pixel<Subpixel = u8> + 'static,
{
    p1.map2(&p2, |a, b| (a as f64 * (1.0 - r) + b as f64 * r).round() as u8)
}
