mod gui;
use nanachi::{image::Rgb, path_outline};
use std::thread;
use std::time::Duration;

fn main() {
    gui::mount_gui(&|render, event_pump| {
        let (width, height) = (640usize, 480usize);
        let mut buffer = nanachi::image::ImageBuffer::from_pixel(
            width as u32,
            height as u32,
            Rgb([255, 255, 255]),
        );

        let color = nanachi::fill_color::Solid::new(Rgb([10, 10, 10]));
        let color2 = nanachi::fill_color::Solid::new(Rgb([200, 10, 10]));

        let mut path: Vec<(f64, f64)> = vec![];
        // nanachi::draw::draw_path(&mut buffer, &path, color, 3.0);

        'running: loop {
            render(&buffer);
            for ev in event_pump.poll_iter() {
                println!("{:?}", ev);
                match ev {
                    gui::Event::MouseButtonDown { x, y, .. } => {
                        buffer = nanachi::image::ImageBuffer::from_pixel(
                            width as u32,
                            height as u32,
                            Rgb([255, 255, 255]),
                        );
                        path.push((x as f64, y as f64));
                        if 2 <= path.len() {
                            let path2 = nanachi::path3::Path::from_points(&path.iter().map(|x| (*x).into()).collect());
                            let path2 = path_outline::path_outline(&path2, 1.0, &path_outline::Join::Round, &path_outline::Cap::Round);
                            draw_fill(&mut buffer, &nanachi::path3::Path(path2), &color);
                        }
                        if 2 <= path.len() {
                            let path2 = path.iter().map(|x| (*x).into()).collect();
                            let path2 = nanachi::path3::Path::from_bezier2_points(&nanachi::k_curve::k_curve(path2, false, 4));
                            let path2 = path_outline::path_outline(&path2, 1.0, &path_outline::Join::Round, &path_outline::Cap::Round);
                            draw_fill(&mut buffer, &nanachi::path3::Path(path2), &color2);
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

fn draw_fill<C: nanachi::fill_color::FillColor<Rgb<u8>>>(
    img: &mut nanachi::image::ImageBuffer<Rgb<u8>, Vec<u8>>,
    path: &nanachi::path3::Path,
    fill_color: &C,
) {
    nanachi::fill_path::draw_fill(
        img.width() as u32,
        img.height() as u32,
        path,
        nanachi::fill_rule::NonZero,
        &mut nanachi::writer::img_writer(img, fill_color, nanachi::compositor::basic::SrcOver),
    );
}
