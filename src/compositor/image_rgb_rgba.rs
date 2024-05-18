use super::*;
use image::{Rgb, Rgba};

macro_rules! def_linear_compositor {
    (
        $name:ident ($aa:ident, $ba:ident => $ca:ident, $ax:ident, $bx:ident)
        {$($rest1:tt)+} {$($rest2:tt)+}
    ) => {
        impl Compositor<Rgba<u8>> for $name {
            #[allow(unused_variables)]
            fn composite_with_alpha(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f32) -> Rgba<u8> {
                let $aa = a.0[3] as u16;
                let $ba = b.0[3] as u16 * (256.0 * alpha).round() as u16 >> 8;
                $($rest1)+
                Rgba([
                    ((a.0[0] as u16 * $ax + 255 >> 8) + (b.0[0] as u16 * $bx + 255 >> 8)).min(255) as u8,
                    ((a.0[1] as u16 * $ax + 255 >> 8) + (b.0[1] as u16 * $bx + 255 >> 8)).min(255) as u8,
                    ((a.0[2] as u16 * $ax + 255 >> 8) + (b.0[2] as u16 * $bx + 255 >> 8)).min(255) as u8,
                    $ca as u8,
                ])
            }
        }

        impl Compositor<Rgb<u8>> for $name {
            #[allow(unused_variables)]
            fn composite_with_alpha(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f32) -> Rgb<u8> {
                let $aa = 255;
                let $ba = (255.0 * alpha).round() as u16;
                $($rest2)+
                Rgb([
                    ((a.0[0] as u16 * $ax + 255 >> 8) + (b.0[0] as u16 * $bx + 255 >> 8)).min(255) as u8,
                    ((a.0[1] as u16 * $ax + 255 >> 8) + (b.0[1] as u16 * $bx + 255 >> 8)).min(255) as u8,
                    ((a.0[2] as u16 * $ax + 255 >> 8) + (b.0[2] as u16 * $bx + 255 >> 8)).min(255) as u8,
                ])
            }
        }
    };
}

def_linear_compositor! {
    Clear(a, b => c, ax, bx) {
        let c = 0;
        let ax = 0;
        let bx = 0;
    } {
        let ax = 0;
        let bx = 0;
    }
}

def_linear_compositor! {
    Src(a, b => c, ax, bx) {
        let c = b;
        let ax = 0;
        let bx = b;
    } {
        let ax = 0;
        let bx = b;
    }
}

def_linear_compositor! {
    Dst(a, b => c, ax, bx) {
        let c = a;
        let ax = a;
        let bx = 0;
    } {
        let ax = a;
        let bx = 0;
    }
}

def_linear_compositor! {
    SrcOver(a, b => c, ax, bx) {
        if a + b == 0 {
            return Rgba([0, 0, 0, 0]);
        }
        let c = a + b - (a * b + 255 >> 8);
        let ax = (a * (255 - b)) / c;
        let bx = (b << 8) / c;
    } {
        let ax = a * (255 - b) + 255 >> 8;
        let bx = b;
    }
}

def_linear_compositor! {
    SrcIn(a, b => c, ax, bx) {
        let c = a * b + 255 >> 8;
        let ax = 0;
        let bx = 255;
    } {
        let ax = 0;
        let bx = 255;
    }
}

def_linear_compositor! {
    SrcOut(a, b => c, ax, bx) {
        let c = (255 - a) * b + 255 >> 8;
        let ax = 0;
        let bx = 255;
    } {
        let ax = 0;
        let bx = 255;
    }
}

def_linear_compositor! {
    SrcAtop(a, b => c, ax, bx) {
        let c = a;
        if c == 0 {
            return Rgba([0, 0, 0, 0]);
        }
        let ax = 255 - b;
        let bx = b;
    } {
        let ax = 255 - b;
        let bx = b;
    }
}

def_linear_compositor! {
    DstOver(a, b => c, ax, bx) {
        let c = a + b - (a * b + 255 >> 8);
        if c == 0 {
            return Rgba([0, 0, 0, 0]);
        }
        let ax = (a << 8) / c;
        let bx = ((255 - a) * b) / c;
    } {
        let ax = a;
        let bx = (255 - a) * b + 255 >> 8;
    }
}

def_linear_compositor! {
    DstIn(a, b => c, ax, bx) {
        let c = a * b + 255 >> 8;
        let ax = 255;
        let bx = 0;
    } {
        let ax = 255;
        let bx = 0;
    }
}

def_linear_compositor! {
    DstOut(a, b => c, ax, bx) {
        let c = a * (255 - b) + 255 >> 8;
        let ax = 255;
        let bx = 0;
    } {
        let ax = 255;
        let bx = 0;
    }
}

def_linear_compositor! {
    DstAtop(a, b => c, ax, bx) {
        let c = b;
        let ax = a;
        let bx = 255 - a;
    } {
        let ax = a;
        let bx = 255 - a;
    }
}

def_linear_compositor! {
    Xor(a, b => c, ax, bx) {
        let ax = a * (255 - b) + 255 >> 8;
        let bx = (255 - a) * b + 255 >> 8;
        let c = a + b - (a * b >> 7);
    } {
        let ax = a * (255 - b) + 255 >> 8;
        let bx = (255 - a) * b + 255 >> 8;
    }
}

def_linear_compositor! {
    Add(a, b => c, ax, bx) {
        let c = (a + b).min(255);
        if c == 0 {
            return Rgba([0, 0, 0, 0]);
        }
        let ax = (a << 8) / c;
        let bx = (b << 8) / c;
    } {
        let ax = a;
        let bx = b;
    }
}

macro_rules! def_compositor {
    (
        $name:ident
        $e:expr
    ) => {
        impl Compositor<Rgba<u8>> for $name {
            #[allow(unused_variables)]
            fn composite_with_alpha(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f32) -> Rgba<u8> {
                let aa = a.0[3] as u16;
                let ba = b.0[3] as u16 * (256.0 * alpha).round() as u16 >> 8;

                let ca = aa + ba - (aa * ba + 255 >> 8);
                if ca == 0 {
                    return Rgba([0, 0, 0, 0]);
                }
                let ax = (aa * (255 - ba)) / ca;
                let bx = (ba * (255 - aa)) / ca;
                let cx = aa * ba / ca;

                let e = $e;
                Rgba([
                    ((a.0[0] as u16 * ax + 255 >> 8)
                        + (b.0[0] as u16 * bx + 255 >> 8)
                        + (e(a.0[0] as u16, b.0[0] as u16) * cx + 255 >> 8))
                        .min(255) as u8,
                    ((a.0[1] as u16 * ax + 255 >> 8)
                        + (b.0[1] as u16 * bx + 255 >> 8)
                        + (e(a.0[1] as u16, b.0[1] as u16) * cx + 255 >> 8))
                        .min(255) as u8,
                    ((a.0[2] as u16 * ax + 255 >> 8)
                        + (b.0[2] as u16 * bx + 255 >> 8)
                        + (e(a.0[2] as u16, b.0[2] as u16) * cx + 255 >> 8))
                        .min(255) as u8,
                    ca as u8,
                ])
            }
        }

        impl Compositor<Rgb<u8>> for $name {
            #[allow(unused_variables)]
            fn composite_with_alpha(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f32) -> Rgb<u8> {
                let aa = 255;
                let ba = (255.0 * alpha).round() as u16;

                let ca = aa + ba - (aa * ba + 255 >> 8);
                if ca == 0 {
                    return Rgb([0, 0, 0]);
                }
                let ax = (aa * (255 - ba)) / ca;
                let bx = (ba * (255 - aa)) / ca;
                let cx = aa * ba / ca;

                let e = $e;
                Rgb([
                    ((a.0[0] as u16 * ax + 255 >> 8)
                        + (b.0[0] as u16 * bx + 255 >> 8)
                        + (e(a.0[0] as u16, b.0[0] as u16) * cx + 255 >> 8))
                        .min(255) as u8,
                    ((a.0[1] as u16 * ax + 255 >> 8)
                        + (b.0[1] as u16 * bx + 255 >> 8)
                        + (e(a.0[1] as u16, b.0[1] as u16) * cx + 255 >> 8))
                        .min(255) as u8,
                    ((a.0[2] as u16 * ax + 255 >> 8)
                        + (b.0[2] as u16 * bx + 255 >> 8)
                        + (e(a.0[2] as u16, b.0[2] as u16) * cx + 255 >> 8))
                        .min(255) as u8,
                ])
            }
        }
    };
}

def_compositor! {
    Darken
    {|a: u16, b: u16| a.min(b)}
}

def_compositor! {
    Lighten
    {|a: u16, b: u16| a.max(b)}
}

def_compositor! {
    Multiply
    {|a: u16, b: u16| (a * b + 255 >> 8)}
}

def_compositor! {
    Screen
    {|a: u16, b: u16| (a + b - (a * b + 255 >> 8))}
}

def_compositor! {
    Overlay
    {|a: u16, b: u16| if a < 128 {a * b + 255 >> 8} else {255 - ((255 - a) * (255 - b) >> 7)}}
}

def_compositor! {
    HardLight
    {|a: u16, b: u16| if b < 128 {a * b + 255 >> 8} else {255 - ((255 - a) * (255 - b) >> 7)}}
}

def_compositor! {
    Dodge
    {|a: u16, b: u16| if b < 255 {((a << 8) / (255 - b)).min(255)} else {255}}
}

def_compositor! {
    Burn
    {|a: u16, b: u16| if 0< b {255 - ((255 - a << 8) / b).min(255)} else {0}}
}

def_compositor! {
    SoftLight
    {|a: u16, b: u16|
        if b < 128 {
            a - (((255 - 2 * b) * a + 255 >> 8) * (255 - a) + 255 >> 8)
        } else {
            let g = if a < 64 {
                ((16 * a - 12 * 256) * a + 4 * 256 >> 8) * a + 255 >> 8
            } else {
                ((a as f32 / 255.0).sqrt() * 255.0).round() as u16
            };
            a + ((2 * b - 256 >> 8) * (g - a) + 255 >> 8)
        }
    }
}

def_compositor! {
    Difference
    {|a: u16, b: u16| (a as i16 - b as i16).abs() as u16}
}

def_compositor! {
    Exclusion
    {|a: u16, b: u16| (a + b - (a * b >> 7))}
}

impl Compositor<Rgba<u8>> for Basic {
    fn composite_with_alpha(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f32) -> Rgba<u8> {
        match self {
            Basic::Clear => Clear.composite_with_alpha(a, b, alpha),
            Basic::Src => Src.composite_with_alpha(a, b, alpha),
            Basic::Dst => Dst.composite_with_alpha(a, b, alpha),
            Basic::SrcOver => SrcOver.composite_with_alpha(a, b, alpha),
            Basic::SrcIn => SrcIn.composite_with_alpha(a, b, alpha),
            Basic::SrcOut => SrcOut.composite_with_alpha(a, b, alpha),
            Basic::SrcAtop => SrcAtop.composite_with_alpha(a, b, alpha),
            Basic::DstOver => DstOver.composite_with_alpha(a, b, alpha),
            Basic::DstIn => DstIn.composite_with_alpha(a, b, alpha),
            Basic::DstOut => DstOut.composite_with_alpha(a, b, alpha),
            Basic::DstAtop => DstAtop.composite_with_alpha(a, b, alpha),
            Basic::Xor => Xor.composite_with_alpha(a, b, alpha),
            Basic::Add => Add.composite_with_alpha(a, b, alpha),
            Basic::Darken => Darken.composite_with_alpha(a, b, alpha),
            Basic::Lighten => Lighten.composite_with_alpha(a, b, alpha),
            Basic::Multiply => Multiply.composite_with_alpha(a, b, alpha),
            Basic::Screen => Screen.composite_with_alpha(a, b, alpha),
            Basic::Overlay => Overlay.composite_with_alpha(a, b, alpha),
            Basic::HardLight => HardLight.composite_with_alpha(a, b, alpha),
            Basic::Dodge => Dodge.composite_with_alpha(a, b, alpha),
            Basic::Burn => Burn.composite_with_alpha(a, b, alpha),
            Basic::SoftLight => SoftLight.composite_with_alpha(a, b, alpha),
            Basic::Difference => Difference.composite_with_alpha(a, b, alpha),
            Basic::Exclusion => Exclusion.composite_with_alpha(a, b, alpha),
        }
    }
}

impl Compositor<Rgb<u8>> for Basic {
    fn composite_with_alpha(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f32) -> Rgb<u8> {
        match self {
            Basic::Clear => Clear.composite_with_alpha(a, b, alpha),
            Basic::Src => Src.composite_with_alpha(a, b, alpha),
            Basic::Dst => Dst.composite_with_alpha(a, b, alpha),
            Basic::SrcOver => SrcOver.composite_with_alpha(a, b, alpha),
            Basic::SrcIn => SrcIn.composite_with_alpha(a, b, alpha),
            Basic::SrcOut => SrcOut.composite_with_alpha(a, b, alpha),
            Basic::SrcAtop => SrcAtop.composite_with_alpha(a, b, alpha),
            Basic::DstOver => DstOver.composite_with_alpha(a, b, alpha),
            Basic::DstIn => DstIn.composite_with_alpha(a, b, alpha),
            Basic::DstOut => DstOut.composite_with_alpha(a, b, alpha),
            Basic::DstAtop => DstAtop.composite_with_alpha(a, b, alpha),
            Basic::Xor => Xor.composite_with_alpha(a, b, alpha),
            Basic::Add => Add.composite_with_alpha(a, b, alpha),
            Basic::Darken => Darken.composite_with_alpha(a, b, alpha),
            Basic::Lighten => Lighten.composite_with_alpha(a, b, alpha),
            Basic::Multiply => Multiply.composite_with_alpha(a, b, alpha),
            Basic::Screen => Screen.composite_with_alpha(a, b, alpha),
            Basic::Overlay => Overlay.composite_with_alpha(a, b, alpha),
            Basic::HardLight => HardLight.composite_with_alpha(a, b, alpha),
            Basic::Dodge => Dodge.composite_with_alpha(a, b, alpha),
            Basic::Burn => Burn.composite_with_alpha(a, b, alpha),
            Basic::SoftLight => SoftLight.composite_with_alpha(a, b, alpha),
            Basic::Difference => Difference.composite_with_alpha(a, b, alpha),
            Basic::Exclusion => Exclusion.composite_with_alpha(a, b, alpha),
        }
    }
}
