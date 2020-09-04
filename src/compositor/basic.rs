use image::{Rgb, Rgba};

pub struct SrcOver;

impl super::Compositor<Rgba<u8>> for SrcOver {
    fn composite(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        let aa = a.0[3] as f64 / std::u8::MAX as f64;
        let ba = b.0[3] as f64 / std::u8::MAX as f64 * alpha;
        let alpha_a = 1.0 - ba;
        let alpha_b = ba;
        Rgba([
            (a.0[0] as f64 * alpha_a + b.0[0] as f64 * alpha_b).round() as u8,
            (a.0[1] as f64 * alpha_a + b.0[1] as f64 * alpha_b).round() as u8,
            (a.0[2] as f64 * alpha_a + b.0[2] as f64 * alpha_b).round() as u8,
            ((aa + ba - aa * ba) * std::u8::MAX as f64).round() as u8,
        ])
    }
}

impl super::Compositor<Rgb<u8>> for SrcOver {
    fn composite(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
        let alpha_a = 1.0 - alpha;
        let alpha_b = alpha;
        Rgb([
            (a.0[0] as f64 * alpha_a + b.0[0] as f64 * alpha_b).round() as u8,
            (a.0[1] as f64 * alpha_a + b.0[1] as f64 * alpha_b).round() as u8,
            (a.0[2] as f64 * alpha_a + b.0[2] as f64 * alpha_b).round() as u8,
        ])
    }
}

pub struct SrcIn;

impl super::Compositor<Rgba<u8>> for SrcIn {
    fn composite(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        let aa = a.0[3] as f64 / std::u8::MAX as f64;
        Rgba([
            b.0[0],
            b.0[1],
            b.0[2],
            (b.0[3] as f64 * alpha * aa).round() as u8,
        ])
    }
}

impl super::Compositor<Rgb<u8>> for SrcIn {
    fn composite(&self, _: &Rgb<u8>, b: &Rgb<u8>, _: f64) -> Rgb<u8> {
        Rgb([
            b.0[0],
            b.0[1],
            b.0[2],
        ])
    }
}

pub struct SrcOut;

impl super::Compositor<Rgba<u8>> for SrcOut {
    fn composite(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        let aa = a.0[3] as f64 / std::u8::MAX as f64;
        Rgba([
            b.0[0],
            b.0[1],
            b.0[2],
            (b.0[3] as f64 * alpha * (1.0 - aa)).round() as u8,
        ])
    }
}

impl super::Compositor<Rgb<u8>> for SrcOut {
    fn composite(&self, _: &Rgb<u8>, b: &Rgb<u8>, _: f64) -> Rgb<u8> {
        Rgb([
            b.0[0],
            b.0[1],
            b.0[2],
        ])
    }
}

pub struct SrcAtop;

impl super::Compositor<Rgba<u8>> for SrcAtop {
    fn composite(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        let ba = b.0[3] as f64 / std::u8::MAX as f64 * alpha;
        let alpha_a = 1.0 - ba;
        let alpha_b = ba;
        Rgba([
            (a.0[0] as f64 * alpha_a + b.0[0] as f64 * alpha_b).round() as u8,
            (a.0[1] as f64 * alpha_a + b.0[1] as f64 * alpha_b).round() as u8,
            (a.0[2] as f64 * alpha_a + b.0[2] as f64 * alpha_b).round() as u8,
            a.0[3],
        ])
    }
}

impl super::Compositor<Rgb<u8>> for SrcAtop {
    fn composite(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
        let alpha_a = 1.0 - alpha;
        let alpha_b = alpha;
        Rgb([
            (a.0[0] as f64 * alpha_a + b.0[0] as f64 * alpha_b).round() as u8,
            (a.0[1] as f64 * alpha_a + b.0[1] as f64 * alpha_b).round() as u8,
            (a.0[2] as f64 * alpha_a + b.0[2] as f64 * alpha_b).round() as u8,
        ])
    }
}
////////////////////////////////////////////
pub struct DstOver;

impl super::Compositor<Rgba<u8>> for DstOver {
    fn composite(&self, b: &Rgba<u8>, a: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        let aa = a.0[3] as f64 / std::u8::MAX as f64 * alpha;
        let ba = b.0[3] as f64 / std::u8::MAX as f64;
        let alpha_a = 1.0 - ba;
        let alpha_b = ba;
        Rgba([
            (a.0[0] as f64 * alpha_a + b.0[0] as f64 * alpha_b).round() as u8,
            (a.0[1] as f64 * alpha_a + b.0[1] as f64 * alpha_b).round() as u8,
            (a.0[2] as f64 * alpha_a + b.0[2] as f64 * alpha_b).round() as u8,
            ((aa + ba - aa * ba) * std::u8::MAX as f64).round() as u8,
        ])
    }
}

impl super::Compositor<Rgb<u8>> for DstOver {
    fn composite(&self, b: &Rgb<u8>, a: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
        let alpha_a = 1.0 - alpha;
        let alpha_b = alpha;
        Rgb([
            (a.0[0] as f64 * alpha_a + b.0[0] as f64 * alpha_b).round() as u8,
            (a.0[1] as f64 * alpha_a + b.0[1] as f64 * alpha_b).round() as u8,
            (a.0[2] as f64 * alpha_a + b.0[2] as f64 * alpha_b).round() as u8,
        ])
    }
}

pub struct DstIn;

impl super::Compositor<Rgba<u8>> for DstIn {
    fn composite(&self, b: &Rgba<u8>, a: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        let aa = a.0[3] as f64 / std::u8::MAX as f64;
        Rgba([
            b.0[0],
            b.0[1],
            b.0[2],
            (b.0[3] as f64 * alpha * aa).round() as u8,
        ])
    }
}

impl super::Compositor<Rgb<u8>> for DstIn {
    fn composite(&self, b: &Rgb<u8>, _: &Rgb<u8>, _: f64) -> Rgb<u8> {
        Rgb([
            b.0[0],
            b.0[1],
            b.0[2],
        ])
    }
}

pub struct DstOut;

impl super::Compositor<Rgba<u8>> for DstOut {
    fn composite(&self, b: &Rgba<u8>, a: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        let aa = a.0[3] as f64 / std::u8::MAX as f64 * alpha;
        Rgba([
            b.0[0],
            b.0[1],
            b.0[2],
            (b.0[3] as f64 * (1.0 - aa)).round() as u8,
        ])
    }
}

impl super::Compositor<Rgb<u8>> for DstOut {
    fn composite(&self, b: &Rgb<u8>, _: &Rgb<u8>, _: f64) -> Rgb<u8> {
        Rgb([
            b.0[0],
            b.0[1],
            b.0[2],
        ])
    }
}

pub struct DstAtop;

impl super::Compositor<Rgba<u8>> for DstAtop {
    fn composite(&self, b: &Rgba<u8>, a: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        let ba = b.0[3] as f64 / std::u8::MAX as f64;
        let alpha_a = 1.0 - ba;
        let alpha_b = ba;
        Rgba([
            (a.0[0] as f64 * alpha_a + b.0[0] as f64 * alpha_b).round() as u8,
            (a.0[1] as f64 * alpha_a + b.0[1] as f64 * alpha_b).round() as u8,
            (a.0[2] as f64 * alpha_a + b.0[2] as f64 * alpha_b).round() as u8,
            (a.0[3] as f64 * alpha).round() as u8,
        ])
    }
}

impl super::Compositor<Rgb<u8>> for DstAtop {
    fn composite(&self, b: &Rgb<u8>, a: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
        let alpha_a = 1.0 - alpha;
        let alpha_b = alpha;
        Rgb([
            (a.0[0] as f64 * alpha_a + b.0[0] as f64 * alpha_b).round() as u8,
            (a.0[1] as f64 * alpha_a + b.0[1] as f64 * alpha_b).round() as u8,
            (a.0[2] as f64 * alpha_a + b.0[2] as f64 * alpha_b).round() as u8,
        ])
    }
}

pub struct Xor;

impl super::Compositor<Rgba<u8>> for Xor {
    fn composite(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        let aa = a.0[3] as f64 / std::u8::MAX as f64;
        let ba = b.0[3] as f64 / std::u8::MAX as f64 * alpha;
        let alpha_a = aa * (1.0 - ba);
        let alpha_b = ba;
        Rgba([
            (a.0[0] as f64 * alpha_a + b.0[0] as f64 * alpha_b).round() as u8,
            (a.0[1] as f64 * alpha_a + b.0[1] as f64 * alpha_b).round() as u8,
            (a.0[2] as f64 * alpha_a + b.0[2] as f64 * alpha_b).round() as u8,
            ((aa + ba - aa * ba * 2.0) * std::u8::MAX as f64).round() as u8,
        ])
    }
}

impl super::Compositor<Rgb<u8>> for Xor {
    fn composite(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
        let alpha_a = 1.0 - alpha;
        let alpha_b = alpha;
        Rgb([
            (a.0[0] as f64 * alpha_a + b.0[0] as f64 * alpha_b).round() as u8,
            (a.0[1] as f64 * alpha_a + b.0[1] as f64 * alpha_b).round() as u8,
            (a.0[2] as f64 * alpha_a + b.0[2] as f64 * alpha_b).round() as u8,
        ])
    }
}
