mod gui;
use nanachi::{
    compositor,
    context::{Context, FillStyle},
    fill_color, fill_rule,
    image::Rgba,
    image_crate_adapter::buffer_rgba_f32_to_rgba_image,
};
use std::thread;
use std::time::Duration;

fn main() {
    gui::mount_gui(&|render, event_pump| {
        let (width, height) = (640usize, 480usize);
        let mut buffer = nanachi::image::ImageBuffer::from_pixel(
            width as u32,
            height as u32,
            Rgba([1.0f32, 1.0, 1.0, 1.0]),
        );
        let mut context = Context::from_image(&mut buffer);

        let color = nanachi::fill_color::Solid::new(Rgba([0.1f32, 0.1, 0.1, 1.0]));
        let color2 = nanachi::fill_color::Solid::new(Rgba([0.9f32, 0.0, 0.0, 1.0]));

        let mut points: Vec<(f64, f64)> = vec![];

        'running: loop {
            render(&buffer_rgba_f32_to_rgba_image(&context.image.clone()));
            for ev in event_pump.poll_iter() {
                println!("{:?}", ev);
                match ev {
                    gui::Event::MouseButtonDown { x, y, .. } => {
                        context.clear(&fill_color::Solid::new(Rgba([1.0, 1.0, 1.0, 1.0])));
                        points.push((x as f64, y as f64));
                        if 2 <= points.len() {
                            let path = nanachi::path::Path::from_points(
                                &points.iter().map(|x| (*x).into()).collect(),
                            );
                            context.stroke(
                                &path,
                                &FillStyle {
                                    color: color.clone(),
                                    compositor: compositor::SrcOver,
                                    fill_rule: fill_rule::NonZero,
                                    pixel: Default::default(),
                                },
                                2.0,
                            );
                        }
                        if 2 <= points.len() {
                            let path = points.iter().map(|x| (*x).into()).collect();
                            let path = nanachi::path::Path::from_bezier2_points(
                                &nanachi::k_curve::k_curve(path, false, 4),
                            );
                            context.stroke(
                                &path,
                                &FillStyle {
                                    color: color2.clone(),
                                    compositor: compositor::SrcOver,
                                    fill_rule: fill_rule::NonZero,
                                    pixel: Default::default(),
                                },
                                2.0,
                            );
                        }
                    }
                    gui::Event::Quit { .. }
                    | gui::Event::KeyDown {
                        keycode: Some(gui::Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    });
}
