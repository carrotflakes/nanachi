use image::{Rgb, Rgba};

pub struct Normal;

impl super::Compositor<Rgba<u8>> for Normal {
    fn composite(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        let aa = a.0[3] as f64 / std::u8::MAX as f64;
        let ba = b.0[3] as f64 / std::u8::MAX as f64 * alpha;
        let det = aa + ba + 0.00001;
        let alpha_a = aa / det;
        let alpha_b = ba / det;
        Rgba([
            (a.0[0] as f64 * alpha_a + b.0[0] as f64 * alpha_b).round() as u8,
            (a.0[1] as f64 * alpha_a + b.0[1] as f64 * alpha_b).round() as u8,
            (a.0[2] as f64 * alpha_a + b.0[2] as f64 * alpha_b).round() as u8,
            ((aa + ba - aa * ba) * std::u8::MAX as f64).round() as u8,
        ])
    }
}

impl super::Compositor<Rgb<u8>> for Normal {
    fn composite(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
        let ai = 1.0 - alpha;
        Rgb([
            (a.0[0] as f64 * ai + b.0[0] as f64 * alpha).round() as u8,
            (a.0[1] as f64 * ai + b.0[1] as f64 * alpha).round() as u8,
            (a.0[2] as f64 * ai + b.0[2] as f64 * alpha).round() as u8,
        ])
    }
}
