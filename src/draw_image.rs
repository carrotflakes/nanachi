use crate::buffer::Buffer;
use crate::compositor::Compositor;
use crate::interpolation::Interpolation;
use crate::matrix::Matrix2d;
use crate::pixel::Pixel;

/// Draw `src` image to `dst` image.
pub fn draw_image_pixel_perfect<P, BD, BS, C>(
    dst: &mut BD,
    src: &BS,
    dst_pos: (u32, u32),
    src_pos: (u32, u32),
    size: (u32, u32),
    compositor: &C,
) where
    P: Pixel,
    BD: Buffer<P>,
    BS: Buffer<P>,
    C: Compositor<P>,
{
    for dy in 0..size.1 {
        for dx in 0..size.0 {
            let sx = src_pos.0 + dx;
            let sy = src_pos.1 + dy;
            let dst_x = dst_pos.0 + dx;
            let dst_y = dst_pos.1 + dy;
            let dp = dst.get_pixel_mut(dst_x, dst_y);
            *dp = compositor.composite(dp, src.get_pixel(sx, sy), 1.0);
        }
    }
}

pub fn draw_image_transformed<P, BD, BS, C, I>(
    dst: &mut BD,
    src: &BS,
    src_rect: (f64, f64, f64, f64), // x1, y1, x2, y2
    matrix: Matrix2d,
    compositor: &C,
    interpolation: I,
) where
    P: Pixel,
    BD: Buffer<P>,
    BS: Buffer<P>,
    C: Compositor<P>,
    I: Interpolation<P, BS>,
{
    let src_size = (src_rect.2 - src_rect.0, src_rect.3 - src_rect.1);
    let (left, top, right, bottom) = [
        matrix.apply((0.0f64, 0.0f64)),
        matrix.apply((src_size.0, 0.0f64)),
        matrix.apply((0.0f64, src_size.1)),
        matrix.apply((src_size.0, src_size.1)),
    ]
    .iter()
    .fold(
        (std::f64::INFINITY, std::f64::INFINITY, 0.0f64, 0.0f64),
        |a, b| (a.0.min(b.0), a.1.min(b.1), a.2.max(b.0), a.3.max(b.1)),
    );
    let inverted_matrix = matrix.inverse();
    let dst_size = dst.dimensions();

    for y in top.floor().max(0.0) as u32..(bottom.ceil() as u32).min(dst_size.1) {
        for x in left.floor().max(0.0) as u32..(right.ceil() as u32).min(dst_size.0) {
            let sp = inverted_matrix.apply((x as f64, y as f64));
            let (src_x, src_y) = (sp.0.round() as i32, sp.1.round() as i32);
            if 0 <= src_x && src_x < src_size.0 as i32 && 0 <= src_y && src_y < src_size.1 as i32 {
                let dp = dst.get_pixel_mut(x, y);
                let sp = interpolation.interpolate(src, sp.0, sp.1);
                *dp = compositor.composite(dp, &sp, 1.0);
            }
        }
    }
}
