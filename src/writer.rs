use crate::fill_color::FillColor;
use crate::compositor::Compositor;
use image::{ImageBuffer, Pixel};

pub fn alpha_blend<'a, X, C: FillColor<X>>(
    buf: &'a mut ImageBuffer<X, Vec<u8>>,
    fill_color: &'a C,
) -> impl FnMut(u32, u32, f64) + 'a
where
    X: Pixel<Subpixel = u8> + 'static {
    move |x: u32, y: u32, v: f64|
    img_blend_pixel(buf, fill_color, x, y, v)
}

pub fn alpha_blend2<'a, X, C: FillColor<X>>(
    buf: &'a mut ImageBuffer<X, Vec<u8>>,
    fill_color: &'a C,
    alpha: f64,
) -> impl FnMut(u32, u32, f64) + 'a
where
    X: Pixel<Subpixel = u8> + 'static {
    move |x: u32, y: u32, v: f64|
    img_blend_pixel(buf, fill_color, x, y, v * alpha)
}

pub fn img_blend_pixel<X, C: FillColor<X>>(
    buf: &mut ImageBuffer<X, Vec<u8>>,
    fill_color: &C,
    x: u32,
    y: u32,
    r: f64,
) where
    X: Pixel<Subpixel = u8> + 'static,
{
    if x < buf.width() && y < buf.height() {
        let pixel = fill_color.fill_color(x as f64, y as f64);
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

pub fn img_writer<'a, X, F: FillColor<X>, C>(
    buf: &'a mut ImageBuffer<X, Vec<u8>>,
    fill_color: &'a F,
    compositor: C,
) -> impl FnMut(u32, u32, f64) + 'a
where
    X: Pixel<Subpixel = u8> + 'static,
    C: Compositor<X> + 'static,
{
    move |x: u32, y: u32, v: f64| {
        let pixel = fill_color.fill_color(x as f64, y as f64);
        let pixel = compositor.composite(buf.get_pixel(x, y), &pixel, v as f32);
        buf.put_pixel(x, y, pixel);
    }
}
