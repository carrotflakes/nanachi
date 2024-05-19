use crate::buffer::Buffer;
use crate::compositor::Compositor;
use crate::fill_color::FillColor;
use crate::pixel::Pixel;

/// Create a writer that writes the pixel to the buffer.
pub fn image_writer<'a, P, B, F, C>(
    buffer: &'a mut B,
    fill_color: &'a F,
    compositor: &'a C,
) -> impl FnMut(u32, u32, f32) + 'a
where
    P: Pixel,
    B: Buffer<P>,
    F: FillColor<P>,
    C: Compositor<P> + 'static,
{
    let composite = compositor.composite_with_alpha();

    move |x: u32, y: u32, v: f32| {
        let dst = buffer.get_pixel(x, y);
        let src = fill_color.fill_color([x as f32, y as f32]);
        let p = composite(dst, &src, v);
        buffer.put_pixel(x, y, p);
    }
}
