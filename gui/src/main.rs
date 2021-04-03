mod gui;
use nanachi::{
    buffer::GenericBuffer,
    compositor,
    context::{ChildContext, Context, FillStyle},
    fill_color, fill_rule,
    image::RgbaImage,
    matrix::Matrix,
    path_builder::PathBuilder,
    pixel::Pixel,
    pixel::Rgba,
    writer::image_writer,
};
use std::time::Duration;
use std::{thread, time::Instant};

fn main() {
    let fill_style = FillStyle {
        color: fill_color::Solid::new(Rgba([0.0f32, 0.0, 0.0, 1.0])),
        compositor: compositor::SrcOver,
        fill_rule: fill_rule::NonZero,
        pixel: Default::default(),
    };

    gui::mount_gui(&|render, event_pump| {
        let (width, height) = (640usize, 480usize);
        let mut context =
            Context::from_pixel(width as u32, height as u32, Rgba([1.0f32, 1.0, 1.0, 1.0]))
                .high_quality();

        let color1 = fill_color::Solid::new(Rgba([0.1f32, 0.1, 0.1, 1.0]));
        let color2 = fill_color::Solid::new(Rgba([0.9f32, 0.0, 0.0, 1.0]));
        let fill_style1 = FillStyle {
            color: color1,
            ..fill_style.clone()
        };
        let fill_style2 = FillStyle {
            color: color2,
            compositor: compositor::SrcOver,
            fill_rule: fill_rule::NonZero,
            pixel: Default::default(),
        };

        let mut points: Vec<(f64, f64)> = vec![];

        let circle = {
            let mut pb = PathBuilder::new();
            pb.arc(0.0, 0.0, 10.0, 0.0, 360.0f64.to_radians());
            pb.end()
        };
        let circle_style = FillStyle {
            color: fill_color::Solid::new(Rgba([1.0f32, 0.0, 0.0, 0.7])),
            compositor: compositor::SrcOver,
            fill_rule: fill_rule::NonZero,
            pixel: Default::default(),
        };

        let mut count = 0;
        let mut tk = TimeKeeper::new();

        'running: loop {
            context.clear(&fill_color::Solid::new(Rgba([1.0, 1.0, 1.0, 1.0])));
            context
                .transformed_context(
                    &Matrix::new()
                        .scale(((count as f64) * 10.0).to_radians().sin(), 1.0)
                        .translate(12.0, 12.0),
                )
                .fill(&circle, &circle_style);
            draw_text(
                context.child(),
                format!("fps: {:0.3}", tk.actual_fps()).as_str(),
            );
            count += 1;
            if 2 <= points.len() {
                let path = nanachi::path::Path::from_points(
                    &points.iter().map(|x| (*x).into()).collect(),
                    false,
                );
                context.stroke(&path, &fill_style1, 2.0);
            }
            if 2 <= points.len() {
                let path = points.iter().map(|x| (*x).into()).collect();
                let path = nanachi::path::Path::from_bezier2_points(&nanachi::k_curve::k_curve(
                    path, false, 4,
                ));
                context.stroke(&path, &fill_style2, 2.0);
            }
            let img: RgbaImage = (&context.image).into();
            render(&img);
            for ev in event_pump.poll_iter() {
                println!("{:?}", ev);
                match ev {
                    gui::Event::MouseButtonDown { x, y, .. } => {
                        points.push((x as f64, y as f64));
                    }
                    gui::Event::Quit { .. }
                    | gui::Event::KeyDown {
                        keycode: Some(gui::Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            tk.sleep();
        }
    });
}

struct TimeKeeper {
    last: Instant,
    fps: u32,
    history: Vec<Instant>,
}

impl TimeKeeper {
    pub fn new() -> TimeKeeper {
        TimeKeeper {
            last: Instant::now(),
            fps: 30,
            history: vec![Instant::now()],
        }
    }

    pub fn sleep(&mut self) {
        self.last += Duration::new(0, 1_000_000_000u32 / self.fps);
        let now = Instant::now();
        thread::sleep(self.last.saturating_duration_since(now));
        self.history.push(now);
        if 30 < self.history.len() {
            self.history.remove(0);
        }
    }

    pub fn actual_fps(&self) -> f64 {
        ((*self.history.last().unwrap() - self.history[0]).as_secs_f64()
            / self.history.len() as f64)
            .recip()
    }
}

fn draw_text<'a>(context: ChildContext<'a, Rgba, GenericBuffer<Rgba>>, text: &str) {
    use rusttype::{point, Font, Scale};
    let color = fill_color::Solid::new(Rgba([0.0, 0.0, 0.0, 1.0]));
    let compositor = compositor::SrcOver;
    let bytes = include_bytes!("../clacon.ttf");
    let font = Font::try_from_bytes(bytes).unwrap();

    let scale = Scale::uniform(20.0);
    let v_metrics = font.v_metrics(scale);

    let start = point(0.0, v_metrics.ascent);
    let glyphs: Vec<_> = font.layout(text, scale, start).collect();

    // get bounding box
    // let (left, top, right, bottom) = glyphs.iter().fold((500, 100, 0, 0), |b, g| {
    //     if let Some(bb) = g.pixel_bounding_box() {
    //         (b.0.min(bb.min.x), b.1.min(bb.min.y), b.2.max(bb.max.x), b.3.max(bb.max.y))
    //     } else {
    //         b
    //     }
    // });

    // render text
    let mut write = image_writer(context.image, &color, &compositor);
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
