use image::Rgb;

pub struct Normal {
    pub alpha: f64,
}

impl super::Compositor<Rgb<u8>> for Normal {
    fn composite(&self, a: &Rgb<u8>, b: &Rgb<u8>) -> Rgb<u8> {
        Rgb([
            (a.0[0] as f64 * (1.0 - self.alpha) + b.0[0] as f64 * self.alpha).round() as u8,
            (a.0[1] as f64 * (1.0 - self.alpha) + b.0[1] as f64 * self.alpha).round() as u8,
            (a.0[2] as f64 * (1.0 - self.alpha) + b.0[2] as f64 * self.alpha).round() as u8,
        ])
    }
}
