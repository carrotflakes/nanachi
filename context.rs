use image::{ImageBuffer, Rgb};

pub struct Context<'a> {
    buffer: &'a Buffer,
}

impl<'a> Context<'a> {
    pub fn new(buffer: &'a Buffer) -> Context<'a> {
        Context {buffer}
    }
    
    // pub fn draw_path(&mut self, ps: &[(f64, f64)], pixel: Rgba) {
    //     let buffer = &mut self.buffer;
    //     for y in 0..buffer.get_size().1 {
    //         for x in 0..buffer.get_size().0 {
    //             for pair in ps.windows(2) {
    //                 let d = distance_between_line_segment_and_point(&pair[0], &pair[1], &(x as f64, y as f64));
    //                 if d < 5.0 {
    //                     img.put_pixel(x, y, pixel);
    //                 } else if d < 6.0 {
    //                     img.put_pixel(x, y, blend_rgb(*img.get_pixel(x, y), pixel, 6.0 - d));
    //                 }
    //             }
    //         }
    //     }
    // }
}
