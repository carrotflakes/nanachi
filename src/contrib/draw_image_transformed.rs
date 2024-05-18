use crate::buffer::Buffer;
use crate::compositor::Compositor;
use crate::interpolation::Interpolation;
use crate::matrix::Matrix;
use crate::pixel::Pixel;

pub fn draw_image_transformed<P, BD, BS, C, I>(
    dst: &mut BD,
    src: &BS,
    src_rect: [f32; 4], // x1, y1, x2, y2
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
    let matrix = Matrix::new()
        .translate(-src_rect[0], -src_rect[1])
        .then(&matrix);

    let [left, top, right, bottom]: [f32; 4] = [
        matrix.apply((src_rect[0], src_rect[1])),
        matrix.apply((src_rect[2], src_rect[1])),
        matrix.apply((src_rect[0], src_rect[3])),
        matrix.apply((src_rect[2], src_rect[3])),
    ]
    .iter()
    .fold(
        [std::f32::INFINITY, std::f32::INFINITY, 0.0, 0.0],
        |a, b| [a[0].min(b.0), a[1].min(b.1), a[2].max(b.0), a[3].max(b.1)],
    );
    let inverted_matrix = matrix.inverse();
    let dst_size = dst.dimensions();

    for y in top.floor().max(0.0) as u32..(bottom.ceil() as u32).min(dst_size.1) {
        for x in left.floor().max(0.0) as u32..(right.ceil() as u32).min(dst_size.0) {
            let sp = inverted_matrix.apply([x as f32, y as f32]);
            if src_rect[0] <= sp[0]
                && sp[0] < src_rect[2]
                && src_rect[1] <= sp[1]
                && sp[1] < src_rect[3]
            {
                let dp = dst.get_pixel(x, y);
                let sp = interpolation.interpolate(src, sp.into());
                let p = compositor.composite(dp, &sp);
                dst.put_pixel(x, y, p);
            }
        }
    }
}
