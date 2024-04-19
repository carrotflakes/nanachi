use crate::buffer::Buffer;
use crate::compositor::Compositor;
use crate::interpolation::Interpolation;
use crate::matrix::Matrix;
use crate::pixel::Pixel;

pub fn draw_image_transformed<P, BD, BS, C, I>(
    dst: &mut BD,
    src: &BS,
    src_rect: (f32, f32, f32, f32), // x1, y1, x2, y2
    matrix: Matrix,
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
    let (left, top, right, bottom): (f32, f32, f32, f32) = [
        matrix.apply((0.0, 0.0)),
        matrix.apply((src_size.0, 0.0)),
        matrix.apply((0.0, src_size.1)),
        matrix.apply((src_size.0, src_size.1)),
    ]
    .iter()
    .fold(
        (std::f32::INFINITY, std::f32::INFINITY, 0.0, 0.0),
        |a, b| (a.0.min(b.0), a.1.min(b.1), a.2.max(b.0), a.3.max(b.1)),
    );
    let inverted_matrix = matrix.inverse();
    let dst_size = dst.dimensions();

    for y in top.floor().max(0.0) as u32..(bottom.ceil() as u32).min(dst_size.1) {
        for x in left.floor().max(0.0) as u32..(right.ceil() as u32).min(dst_size.0) {
            let sp = inverted_matrix.apply((x as f32, y as f32));
            let (src_x, src_y) = (sp.0.round() as i32, sp.1.round() as i32);
            if 0 <= src_x && src_x < src_size.0 as i32 && 0 <= src_y && src_y < src_size.1 as i32 {
                let dp = dst.get_pixel(x, y);
                let sp = interpolation.interpolate(src, sp.0, sp.1);
                let p = compositor.composite(dp, &sp, 1.0);
                dst.put_pixel(x, y, p);
            }
        }
    }
}
