use nanachi::buffer::{Buffer, GenericBuffer};
use nanachi::pixel::{Arithmetic, Pixel, Rgba};

pub enum Extrapolation<P: Pixel + Arithmetic> {
    ExtendEdge,
    Constant(P),
}

pub fn gauss_blur<P: Pixel + Arithmetic, B: Buffer<P>, C: Buffer<P>>(
    buf: &mut B,
    tmp: &mut C,
    radius: f64,
    extrapolation: &Extrapolation<P>,
) {
    let bxs = boxes_for_gauss(radius, 3);
    box_blur(buf, tmp, (bxs[0] - 1) / 2, extrapolation);
    box_blur(buf, tmp, (bxs[1] - 1) / 2, extrapolation);
    box_blur(buf, tmp, (bxs[2] - 1) / 2, extrapolation);
}

fn box_blur<P: Pixel + Arithmetic, B: Buffer<P>, C: Buffer<P>>(
    buf: &mut B,
    tmp: &mut C,
    r: i32,
    extrapolation: &Extrapolation<P>,
) {
    if r <= 0 {
        return;
    }
    box_blur_h(buf, tmp, r, extrapolation);
    box_blur_t(tmp, buf, r, extrapolation);
}

fn box_blur_h<P: Pixel + Arithmetic, B: Buffer<P>, C: Buffer<P>>(
    src: &mut B,
    dst: &mut C,
    r: i32,
    extrapolation: &Extrapolation<P>,
) {
    let (w, h) = src.dimensions();
    let iarr = 1.0 / (r + r + 1) as f32;
    for i in 0..h {
        let (fv, lv) = match extrapolation {
            Extrapolation::ExtendEdge => {
                (src.get_pixel(0, i).clone(), src.get_pixel(w - 1, i).clone())
            }
            Extrapolation::Constant(p) => (p.clone(), p.clone()),
        };
        let mut val = fv.clone() * (r + 1) as f32;
        for j in 0..r {
            val = val + src.get_pixel(j as u32, i as u32).clone();
        }
        for j in 0..=r {
            val = val + src.get_pixel((r + j) as u32, i as u32).clone() - fv.clone();
            dst.put_pixel(j as u32, i as u32, val.clone() * iarr);
        }
        for j in r + 1..w as i32 - r {
            val = val + src.get_pixel((r + j) as u32, i as u32).clone()
                - src.get_pixel((j - r - 1) as u32, i as u32).clone();
            dst.put_pixel(j as u32, i as u32, val.clone() * iarr);
        }
        for j in w as i32 - r..w as i32 {
            val = val + lv.clone() - src.get_pixel((j - r - 1) as u32, i as u32).clone();
            dst.put_pixel(j as u32, i as u32, val.clone() * iarr);
        }
    }
}

fn box_blur_t<P: Pixel + Arithmetic, B: Buffer<P>, C: Buffer<P>>(
    src: &mut B,
    dst: &mut C,
    r: i32,
    extrapolation: &Extrapolation<P>,
) {
    let (w, h) = src.dimensions();
    let iarr = 1.0 / (r + r + 1) as f32;
    for i in 0..w {
        let (fv, lv) = match extrapolation {
            Extrapolation::ExtendEdge => {
                (src.get_pixel(i, 0).clone(), src.get_pixel(i, h - 1).clone())
            }
            Extrapolation::Constant(p) => (p.clone(), p.clone()),
        };
        let mut val = fv.clone() * (r + 1) as f32;
        for j in 0..r {
            val = val + src.get_pixel(i as u32, j as u32).clone();
        }
        for j in 0..=r {
            val = val + src.get_pixel(i as u32, (r + j) as u32).clone() - fv.clone();
            dst.put_pixel(i as u32, j as u32, val.clone() * iarr);
        }
        for j in r + 1..h as i32 - r {
            val = val + src.get_pixel(i as u32, (r + j) as u32).clone()
                - src.get_pixel(i as u32, (j - r - 1) as u32).clone();
            dst.put_pixel(i as u32, j as u32, val.clone() * iarr);
        }
        for j in h as i32 - r..h as i32 {
            val = val + lv.clone() - src.get_pixel(i as u32, (j - r - 1) as u32).clone();
            dst.put_pixel(i as u32, j as u32, val.clone() * iarr);
        }
    }
}

fn boxes_for_gauss(sigma: f64, n: i32) -> Vec<i32> {
    let w_ideal = ((12.0 * sigma * sigma / n as f64) + 1.0).sqrt();
    let mut wl = w_ideal.floor() as i32;
    if wl % 2 == 0 {
        wl -= 1;
    }
    let wu = wl + 2;

    let m_ideal =
        (12.0 * sigma * sigma - (n * wl * wl + 4 * n * wl + 3 * n) as f64) / (-4 * wl - 4) as f64;
    let m = m_ideal.round() as i32;

    (0..n).map(|i| if i < m { wl } else { wu }).collect()
}

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("nanachi.png".to_string());
    let src = image::open(file).unwrap().into_rgba();
    let (width, height) = src.dimensions();
    let mut img = GenericBuffer::from_pixel(width, height, Rgba::zero());
    for y in 0..height {
        for x in 0..width {
            let p = src.get_pixel(x, y).0;
            img.put_pixel(
                x,
                y,
                Rgba([
                    p[0] as f32 / 255.0,
                    p[1] as f32 / 255.0,
                    p[2] as f32 / 255.0,
                    p[3] as f32 / 255.0,
                ]),
            );
        }
    }

    let mut tmp = GenericBuffer::from_pixel(width, height, Rgba::zero());
    let t = std::time::Instant::now();
    gauss_blur(&mut img, &mut tmp, 10.0, &Extrapolation::ExtendEdge);
    // box_blur(&mut img, &mut tmp, 10);
    dbg!(t.elapsed());
    let img: image::RgbaImage = (&img).into();
    img.save("fast_gauss_blur.png").unwrap();
}
