use image::{Rgb, Rgba};
use super::Compositor;

macro_rules! def_linear_compositor {
    (
        $name:ident ($aa:ident, $ba:ident => $ca:ident, $ax:ident, $bx:ident)
        {$($rest1:tt)+} {$($rest2:tt)+}
    ) => {
        pub struct $name;

        impl Compositor<Rgba<u8>> for $name {
            #[allow(unused_variables)]
            fn composite(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
                let $aa = a.0[3] as f64 / std::u8::MAX as f64;
                let $ba = b.0[3] as f64 / std::u8::MAX as f64 * alpha;
                $($rest1)+
                Rgba([
                    (a.0[0] as f64 * $ax + b.0[0] as f64 * $bx).round() as u8,
                    (a.0[1] as f64 * $ax + b.0[1] as f64 * $bx).round() as u8,
                    (a.0[2] as f64 * $ax + b.0[2] as f64 * $bx).round() as u8,
                    ($ca * std::u8::MAX as f64).round() as u8,
                ])
            }
        }

        impl Compositor<Rgb<u8>> for $name {
            #[allow(unused_variables)]
            fn composite(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
                let $aa = 1.0;
                let $ba = alpha;
                $($rest2)+
                Rgb([
                    (a.0[0] as f64 * $ax + b.0[0] as f64 * $bx).round() as u8,
                    (a.0[1] as f64 * $ax + b.0[1] as f64 * $bx).round() as u8,
                    (a.0[2] as f64 * $ax + b.0[2] as f64 * $bx).round() as u8,
                ])
            }
        }
    };
}

def_linear_compositor!(
    SrcOver(a, b => c, ax, bx) {
        let c = a + b - a * b;
        let ax = (a * (1.0 - b)) / c;
        let bx = b / c;
    } {
        let ax = a * (1.0 - b);
        let bx = b;
    }
);

def_linear_compositor!(
    SrcIn(a, b => c, ax, bx) {
        let c = a * b;
        let ax = 0.0;
        let bx = 1.0;
    } {
        let ax = 0.0;
        let bx = 1.0;
    }
);

def_linear_compositor!(
    SrcOut(a, b => c, ax, bx) {
        let c = (1.0 - a) * b;
        let ax = 0.0;
        let bx = 1.0;
    } {
        let ax = 0.0;
        let bx = 1.0;
    }
);

def_linear_compositor!(
    SrcAtop(a, b => c, ax, bx) {
        let c = b;
        let ax = 1.0 - b;
        let bx = b;
    } {
        let ax = 1.0 - b;
        let bx = b;
    }
);

def_linear_compositor!(
    DstOver(a, b => c, ax, bx) {
        let c = a + b - a * b;
        let ax = a / c;
        let bx = ((1.0 - a) * b) / c;
    } {
        let ax = a;
        let bx = (1.0 - a) * b;
    }
);

def_linear_compositor!(
    DstIn(a, b => c, ax, bx) {
        let c = a * b;
        let ax = 1.0;
        let bx = 0.0;
    } {
        let ax = 1.0;
        let bx = 0.0;
    }
);

def_linear_compositor!(
    DstOut(a, b => c, ax, bx) {
        let c = a * (1.0 - b);
        let ax = 1.0;
        let bx = 0.0;
    } {
        let ax = 1.0;
        let bx = 0.0;
    }
);

def_linear_compositor!(
    DstAtop(a, b => c, ax, bx) {
        let c = a;
        let ax = a;
        let bx = 1.0 - a;
    } {
        let ax = a;
        let bx = 1.0 - a;
    }
);

def_linear_compositor!(
    Xor(a, b => c, ax, bx) {
        let ax = a * (1.0 - b);
        let bx = (1.0 - a) * b;
        let c = a + b - 2.0 * a * b;
    } {
        let ax = a * (1.0 - b);
        let bx = (1.0 - a) * b;
    }
);

macro_rules! def_compositor {
    (
        $name:ident ($a:ident, $b:ident, $alpha:ident)
        {$($rest1:tt)+} {$($rest2:tt)+}
    ) => {
        pub struct $name;

        impl Compositor<Rgba<u8>> for $name {
            #[allow(unused_variables)]
            fn composite(&self, $a: &Rgba<u8>, $b: &Rgba<u8>, $alpha: f64) -> Rgba<u8> {
                $($rest1)+
            }
        }

        impl Compositor<Rgb<u8>> for $name {
            #[allow(unused_variables)]
            fn composite(&self, $a: &Rgb<u8>, $b: &Rgb<u8>, $alpha: f64) -> Rgb<u8> {
                $($rest2)+
            }
        }
    };
}

def_compositor!{
    Darken(a, b, alpha) {
        let aa = a.0[3] as f64 / std::u8::MAX as f64;
        let ba = b.0[3] as f64 / std::u8::MAX as f64 * alpha;
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba;
        Rgba([
            (a.0[0] as f64 * ax + b.0[0] as f64 * bx + a.0[0].min(b.0[0]) as f64 * cx).round() as u8,
            (a.0[1] as f64 * ax + b.0[1] as f64 * bx + a.0[1].min(b.0[1]) as f64 * cx).round() as u8,
            (a.0[2] as f64 * ax + b.0[2] as f64 * bx + a.0[2].min(b.0[2]) as f64 * cx).round() as u8,
            (ca * std::u8::MAX as f64).round() as u8,
        ])
    } {
        let aa = 1.0;
        let ba = alpha;
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba;
        Rgb([
            (a.0[0] as f64 * ax + b.0[0] as f64 * bx + a.0[0].min(b.0[0]) as f64 * cx).round() as u8,
            (a.0[1] as f64 * ax + b.0[1] as f64 * bx + a.0[1].min(b.0[1]) as f64 * cx).round() as u8,
            (a.0[2] as f64 * ax + b.0[2] as f64 * bx + a.0[2].min(b.0[2]) as f64 * cx).round() as u8,
        ])
    }
}

def_compositor!{
    Lighten(a, b, alpha) {
        let aa = a.0[3] as f64 / std::u8::MAX as f64;
        let ba = b.0[3] as f64 / std::u8::MAX as f64 * alpha;
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba;
        Rgba([
            (a.0[0] as f64 * ax + b.0[0] as f64 * bx + a.0[0].max(b.0[0]) as f64 * cx).round() as u8,
            (a.0[1] as f64 * ax + b.0[1] as f64 * bx + a.0[1].max(b.0[1]) as f64 * cx).round() as u8,
            (a.0[2] as f64 * ax + b.0[2] as f64 * bx + a.0[2].max(b.0[2]) as f64 * cx).round() as u8,
            (ca * std::u8::MAX as f64).round() as u8,
        ])
    } {
        let aa = 1.0;
        let ba = alpha;
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba;
        Rgb([
            (a.0[0] as f64 * ax + b.0[0] as f64 * bx + a.0[0].max(b.0[0]) as f64 * cx).round() as u8,
            (a.0[1] as f64 * ax + b.0[1] as f64 * bx + a.0[1].max(b.0[1]) as f64 * cx).round() as u8,
            (a.0[2] as f64 * ax + b.0[2] as f64 * bx + a.0[2].max(b.0[2]) as f64 * cx).round() as u8,
        ])
    }
}
