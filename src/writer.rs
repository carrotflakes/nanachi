use crate::buffer::Buffer;
use crate::compositor::Compositor;
use crate::fill_color::FillColor;
use crate::pixel::Pixel;

pub fn img_writer<'a, P, B, F: FillColor<P>, C>(
    buffer: &'a mut B,
    fill_color: &'a F,
    compositor: &'a C,
) -> impl FnMut(u32, u32, f64) + 'a
where
    P: Pixel,
    B: Buffer<P>,
    C: Compositor<P> + 'static,
{
    move |x: u32, y: u32, v: f64| {
        let dst = buffer.get_pixel_mut(x, y);
        let src = fill_color.fill_color(x as f64, y as f64);
        *dst = compositor.composite(dst, &src, v);
    }
}
