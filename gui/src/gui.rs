use nanachi::image::{ImageBuffer, Rgb};
use sdl2::pixels::{Color, PixelFormatEnum};
pub use sdl2::{event::Event, keyboard::Keycode, EventPump};

pub fn mount_gui(f: &dyn Fn(&mut dyn FnMut(&ImageBuffer<Rgb<u8>, Vec<u8>>), &mut EventPump)) {
    let (width, height) = (640usize, 480usize);
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsys = sdl_ctx.video().unwrap();
    let window = video_subsys
        .window("SDL2", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32)
        .unwrap();

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    let mut render = |buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>| {
        texture
            .with_lock(None, |buf: &mut [u8], pitch: usize| {
                for y in 0..height {
                    for x in 0..width {
                        let offset = y * pitch + x * 3;

                        let p = buffer.get_pixel(x as u32, y as u32);
                        buf[offset] = p.0[0];
                        buf[offset + 1] = p.0[1];
                        buf[offset + 2] = p.0[2];
                    }
                }
            })
            .unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.clear();
        canvas
            .copy(
                &texture,
                None,
                Some(sdl2::rect::Rect::new(0, 0, width as u32, height as u32)),
            )
            .unwrap();
        canvas.present();
    };

    f(&mut render, &mut event_pump);
}
