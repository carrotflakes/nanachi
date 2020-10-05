use image::RgbaImage;
use nanachi::{
    compositor,
    context::{Context, FillStyle},
    fill_color, fill_rule,
    pixel::Rgba,
    primitives::rect,
    writer::img_writer,
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

        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = glyphs.last().unwrap().pixel_bounding_box().unwrap().max.x
            - glyphs.first().unwrap().pixel_bounding_box().unwrap().min.x;

        // show rectangle
        context.fill(
            &rect(
                glyphs[0].pixel_bounding_box().unwrap().min.x as f64,
                v_metrics.descent as f64,
                glyphs_width as f64,
                glyphs_height as f64,
            ),
            &rect_fill_style,
        );

        // render text
        let mut write = img_writer(&mut context.image, &color, &compositor);
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x + bounding_box.min.x as u32;
                    let y = y + bounding_box.min.y as u32;
                    write(x, y, v.min(1.0) as f64);
                });
            }
        }
    }

    let img: RgbaImage = (&context.image).into();
    img.save("./text.png").unwrap();
}
