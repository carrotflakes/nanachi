use crate::fill_color::FillColor;
use crate::compositor::Compositor;
use image::{ImageBuffer, Pixel};

pub fn img_writer<'a, X, F: FillColor<X>, C>(
    buf: &'a mut ImageBuffer<X, Vec<u8>>,
    fill_color: &'a F,
    compositor: &'a C,
) -> impl FnMut(u32, u32, f64) + 'a
where
    X: Pixel<Subpixel = u8> + 'static,
    C: Compositor<X> + 'static,
{
    move |x: u32, y: u32, v: f64| {
        let pixel = fill_color.fill_color(x as f64, y as f64);
        let pixel = compositor.composite(buf.get_pixel(x, y), &pixel, v);
        buf.put_pixel(x, y, pixel);
    }
}
