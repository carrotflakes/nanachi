mod gui;
use nanachi::image::Rgb;
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

        let color = Rgb([10, 10, 10]);
        let color2 = Rgb([200, 10, 10]);

        let mut path: Vec<(f64, f64)> = vec![];
        nanachi::draw::draw_path(&mut buffer, &path, color, 3.0);

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
                        nanachi::draw::draw_path(&mut buffer, &path, color, 3.0);
                        let path2 = path.iter().map(|x| (*x).into()).collect();
                        let path2 = nanachi::k_curve::k_curve(path2, false, 4).as_lines_points(10);
                        nanachi::draw::draw_path(&mut buffer, &path2, color2, 1.0);
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
