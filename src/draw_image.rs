use crate::buffer::Buffer;
use crate::compositor::Compositor;
use crate::pixel::Pixel;

/// Draw `src` image to `dst` image.
pub fn draw_image_pixel_perfect<P, BD, BS, C>(
    dst: &mut BD,
    src: &BS,
    dst_pos: [u32; 2],
    src_pos: [u32; 2],
    size: [u32; 2],
    compositor: &C,
) where
    P: Pixel,
    BD: Buffer<P>,
    BS: Buffer<P>,
    C: Compositor<P>,
{
    let composite = compositor.composite();

    for dy in 0..size[1] {
        for dx in 0..size[0] {
            let sx = src_pos[0] + dx;
            let sy = src_pos[1] + dy;
            let dst_x = dst_pos[0] + dx;
            let dst_y = dst_pos[1] + dy;
            let dp = dst.get_pixel(dst_x, dst_y);
            let p = composite(dp, src.get_pixel(sx, sy));
            dst.put_pixel(dst_x, dst_y, p);
        }
    }
}
