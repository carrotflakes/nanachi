use crate::compositor::Compositor;
use image::{ImageBuffer, Pixel};

pub fn draw_image_pixel_perfect<X, C>(
    dst: &mut ImageBuffer<X, Vec<u8>>,
    src: &ImageBuffer<X, Vec<u8>>,
    dst_pos: (u32, u32),
    src_pos: (u32, u32),
    size: (u32, u32),
    compositor: &C,
)
where
    X: Pixel<Subpixel = u8> + 'static,
    C: Compositor<X> + 'static,
{
    for dy in 0..size.1 {
        for dx in 0..size.0 {
            let sx = src_pos.0 + dx;
            let sy = src_pos.1 + dy;
            let dst_x = dst_pos.0 + dx;
            let dst_y = dst_pos.1 + dy;
            let dp = dst.get_pixel_mut(dst_x, dst_y);
            *dp = compositor.composite(src.get_pixel(sx, sy), dp, 1.0);
        }
    }
}
