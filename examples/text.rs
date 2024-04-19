use image::RgbaImage;
use nanachi::{
    compositor,
    context::{Context, FillStyle},
    fill_color, fill_rule,
    pixel::Rgba,
    primitives::rect,
    writer::image_writer,
};
use rusttype::*;

fn main() {
    let (width, height) = (500, 100);

    let mut context = Context::from_pixel(width, height, Rgba([1.0, 1.0, 1.0, 1.0])).high_quality();

    let rect_fill_style = FillStyle {
        color: fill_color::Solid::new(Rgba([0.0, 0.0, 1.0, 0.3])),
        fill_rule: fill_rule::NonZero,
        compositor: compositor::SrcOver,
        pixel: Default::default(),
    };

    {
        let text = "Hello, Nanachi!!";
        let color = fill_color::Solid::new(Rgba([1.0, 0.0, 0.0, 1.0]));
        let compositor = compositor::SrcOver;
        let font_path = "./examples/IPAexfont00401/ipaexg.ttf";
        let bytes = std::fs::read(font_path).unwrap();
        let font = Font::try_from_bytes(&bytes).unwrap();

        let scale = Scale::uniform(40.0);
        let v_metrics = font.v_metrics(scale);

        let start = point(0.0, v_metrics.ascent);
        let glyphs: Vec<_> = font.layout(text, scale, start).collect();

        // get bounding box
        let (left, top, right, bottom) = glyphs.iter().fold((500, 100, 0, 0), |b, g| {
            if let Some(bb) = g.pixel_bounding_box() {
                (
                    b.0.min(bb.min.x),
                    b.1.min(bb.min.y),
                    b.2.max(bb.max.x),
                    b.3.max(bb.max.y),
                )
            } else {
                b
            }
        });

        // show rectangle
        context.fill(
            &rect(
                left as f32,
                top as f32,
                (right - left) as f32,
                (bottom - top) as f32,
            ),
            &rect_fill_style,
        );

        // render text
        let mut write = image_writer(&mut context.image, &color, &compositor);
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x + bounding_box.min.x as u32;
                    let y = y + bounding_box.min.y as u32;
                    write(x, y, v.min(1.0) as f32);
                });
            }
        }
    }

    let img: RgbaImage = (&context.image).into();
    img.save("./text.png").unwrap();
}
