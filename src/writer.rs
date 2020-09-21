use crate::compositor::Compositor;
use crate::fill_color::FillColor;
use crate::pixel::Pixel;
use image::ImageBuffer;

pub fn img_writer<'a, P, F: FillColor<P>, C>(
    buf: &'a mut ImageBuffer<P, Vec<u8>>,
    fill_color: &'a F,
    compositor: &'a C,
) -> impl FnMut(u32, u32, f64) + 'a
where
    P: Pixel,
    C: Compositor<P> + 'static,
{
    move |x: u32, y: u32, v: f64| {
        let pixel = fill_color.fill_color(x as f64, y as f64);
        let pixel = compositor.composite(buf.get_pixel(x, y), &pixel, v);
        buf.put_pixel(x, y, pixel);
    }
}
