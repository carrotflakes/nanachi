use super::*;
use image::{Rgb, Rgba};

macro_rules! def_linear_compositor {
    (
        $name:ident ($aa:ident, $ba:ident => $ca:ident, $ax:ident, $bx:ident)
        {$($rest1:tt)+} {$($rest2:tt)+}
    ) => {
        impl Compositor<Rgba<u8>> for $name {
            type F1 = fn(&Rgba<u8>, &Rgba<u8>, f32) -> Rgba<u8>;
            type F2 = fn(&Rgba<u8>, &Rgba<u8>) -> Rgba<u8>;

            #[allow(unused_variables)]
            fn composite_with_alpha(&self) -> Self::F1 {
                |a, b, alpha| {
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

            #[allow(unused_variables)]
            fn composite(&self) -> Self::F2 {
                |a, b| {
                    let $aa = a.0[3] as u16;
                    let $ba = b.0[3] as u16;
                    $($rest1)+
                    Rgba([
                        ((a.0[0] as u16 * $ax + 255 >> 8) + (b.0[0] as u16 * $bx + 255 >> 8)).min(255) as u8,
                        ((a.0[1] as u16 * $ax + 255 >> 8) + (b.0[1] as u16 * $bx + 255 >> 8)).min(255) as u8,
                        ((a.0[2] as u16 * $ax + 255 >> 8) + (b.0[2] as u16 * $bx + 255 >> 8)).min(255) as u8,
                        $ca as u8,
                    ])
                }
            }
        }

        impl Compositor<Rgb<u8>> for $name {
            type F1 = fn(&Rgb<u8>, &Rgb<u8>, f32) -> Rgb<u8>;
            type F2 = fn(&Rgb<u8>, &Rgb<u8>) -> Rgb<u8>;

            #[allow(unused_variables)]
            fn composite_with_alpha(&self) -> Self::F1 {
                |a, b, alpha| {
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

            #[allow(unused_variables)]
            fn composite(&self) -> Self::F2 {
                |a, b| {
                    let $aa = 255;
                    let $ba = 255;
                    $($rest2)+
                    Rgb([
                        ((a.0[0] as u16 * $ax + 255 >> 8) + (b.0[0] as u16 * $bx + 255 >> 8)).min(255) as u8,
                        ((a.0[1] as u16 * $ax + 255 >> 8) + (b.0[1] as u16 * $bx + 255 >> 8)).min(255) as u8,
                        ((a.0[2] as u16 * $ax + 255 >> 8) + (b.0[2] as u16 * $bx + 255 >> 8)).min(255) as u8,
                    ])
                }
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
            type F1 = fn(&Rgba<u8>, &Rgba<u8>, f32) -> Rgba<u8>;
            type F2 = fn(&Rgba<u8>, &Rgba<u8>) -> Rgba<u8>;

            #[allow(unused_variables)]
            fn composite_with_alpha(&self) -> Self::F1 {
                |a, b, alpha| {
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

            #[allow(unused_variables)]
            fn composite(&self) -> Self::F2 {
                |a, b| {
                    let aa = a.0[3] as u16;
                    let ba = b.0[3] as u16;

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
        }

        impl Compositor<Rgb<u8>> for $name {
            type F1 = fn(&Rgb<u8>, &Rgb<u8>, f32) -> Rgb<u8>;
            type F2 = fn(&Rgb<u8>, &Rgb<u8>) -> Rgb<u8>;

            #[allow(unused_variables)]
            fn composite_with_alpha(&self) -> Self::F1 {
                |a, b, alpha| {
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

            #[allow(unused_variables)]
            fn composite(&self) -> Self::F2 {
                |a, b| {
                    let aa = 255;
                    let ba = 255;

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
            let g = {
                let a = a as f32 / 255.0;
                (
                    if a < 0.25 {
                        ((16.0 * a - 12.0) * a + 4.0) * a
                    } else {
                        a.sqrt()
                    } * 255.0
                ).round() as u16
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
    type F1 = fn(&Rgba<u8>, &Rgba<u8>, f32) -> Rgba<u8>;
    type F2 = fn(&Rgba<u8>, &Rgba<u8>) -> Rgba<u8>;

    fn composite(&self) -> Self::F2 {
        match self {
            Basic::Clear => Compositor::<Rgba<u8>>::composite(&Clear),
            Basic::Src => Compositor::<Rgba<u8>>::composite(&Src),
            Basic::Dst => Compositor::<Rgba<u8>>::composite(&Dst),
            Basic::SrcOver => Compositor::<Rgba<u8>>::composite(&SrcOver),
            Basic::SrcIn => Compositor::<Rgba<u8>>::composite(&SrcIn),
            Basic::SrcOut => Compositor::<Rgba<u8>>::composite(&SrcOut),
            Basic::SrcAtop => Compositor::<Rgba<u8>>::composite(&SrcAtop),
            Basic::DstOver => Compositor::<Rgba<u8>>::composite(&DstOver),
            Basic::DstIn => Compositor::<Rgba<u8>>::composite(&DstIn),
            Basic::DstOut => Compositor::<Rgba<u8>>::composite(&DstOut),
            Basic::DstAtop => Compositor::<Rgba<u8>>::composite(&DstAtop),
            Basic::Xor => Compositor::<Rgba<u8>>::composite(&Xor),
            Basic::Add => Compositor::<Rgba<u8>>::composite(&Add),
            Basic::Darken => Compositor::<Rgba<u8>>::composite(&Darken),
            Basic::Lighten => Compositor::<Rgba<u8>>::composite(&Lighten),
            Basic::Multiply => Compositor::<Rgba<u8>>::composite(&Multiply),
            Basic::Screen => Compositor::<Rgba<u8>>::composite(&Screen),
            Basic::Overlay => Compositor::<Rgba<u8>>::composite(&Overlay),
            Basic::HardLight => Compositor::<Rgba<u8>>::composite(&HardLight),
            Basic::Dodge => Compositor::<Rgba<u8>>::composite(&Dodge),
            Basic::Burn => Compositor::<Rgba<u8>>::composite(&Burn),
            Basic::SoftLight => Compositor::<Rgba<u8>>::composite(&SoftLight),
            Basic::Difference => Compositor::<Rgba<u8>>::composite(&Difference),
            Basic::Exclusion => Compositor::<Rgba<u8>>::composite(&Exclusion),
        }
    }

    fn composite_with_alpha(&self) -> Self::F1 {
        match self {
            Basic::Clear => Compositor::<Rgba<u8>>::composite_with_alpha(&Clear),
            Basic::Src => Compositor::<Rgba<u8>>::composite_with_alpha(&Src),
            Basic::Dst => Compositor::<Rgba<u8>>::composite_with_alpha(&Dst),
            Basic::SrcOver => Compositor::<Rgba<u8>>::composite_with_alpha(&SrcOver),
            Basic::SrcIn => Compositor::<Rgba<u8>>::composite_with_alpha(&SrcIn),
            Basic::SrcOut => Compositor::<Rgba<u8>>::composite_with_alpha(&SrcOut),
            Basic::SrcAtop => Compositor::<Rgba<u8>>::composite_with_alpha(&SrcAtop),
            Basic::DstOver => Compositor::<Rgba<u8>>::composite_with_alpha(&DstOver),
            Basic::DstIn => Compositor::<Rgba<u8>>::composite_with_alpha(&DstIn),
            Basic::DstOut => Compositor::<Rgba<u8>>::composite_with_alpha(&DstOut),
            Basic::DstAtop => Compositor::<Rgba<u8>>::composite_with_alpha(&DstAtop),
            Basic::Xor => Compositor::<Rgba<u8>>::composite_with_alpha(&Xor),
            Basic::Add => Compositor::<Rgba<u8>>::composite_with_alpha(&Add),
            Basic::Darken => Compositor::<Rgba<u8>>::composite_with_alpha(&Darken),
            Basic::Lighten => Compositor::<Rgba<u8>>::composite_with_alpha(&Lighten),
            Basic::Multiply => Compositor::<Rgba<u8>>::composite_with_alpha(&Multiply),
            Basic::Screen => Compositor::<Rgba<u8>>::composite_with_alpha(&Screen),
            Basic::Overlay => Compositor::<Rgba<u8>>::composite_with_alpha(&Overlay),
            Basic::HardLight => Compositor::<Rgba<u8>>::composite_with_alpha(&HardLight),
            Basic::Dodge => Compositor::<Rgba<u8>>::composite_with_alpha(&Dodge),
            Basic::Burn => Compositor::<Rgba<u8>>::composite_with_alpha(&Burn),
            Basic::SoftLight => Compositor::<Rgba<u8>>::composite_with_alpha(&SoftLight),
            Basic::Difference => Compositor::<Rgba<u8>>::composite_with_alpha(&Difference),
            Basic::Exclusion => Compositor::<Rgba<u8>>::composite_with_alpha(&Exclusion),
        }
    }
}

impl Compositor<Rgb<u8>> for Basic {
    type F1 = fn(&Rgb<u8>, &Rgb<u8>, f32) -> Rgb<u8>;
    type F2 = fn(&Rgb<u8>, &Rgb<u8>) -> Rgb<u8>;

    fn composite(&self) -> Self::F2 {
        match self {
            Basic::Clear => Compositor::<Rgb<u8>>::composite(&Clear),
            Basic::Src => Compositor::<Rgb<u8>>::composite(&Src),
            Basic::Dst => Compositor::<Rgb<u8>>::composite(&Dst),
            Basic::SrcOver => Compositor::<Rgb<u8>>::composite(&SrcOver),
            Basic::SrcIn => Compositor::<Rgb<u8>>::composite(&SrcIn),
            Basic::SrcOut => Compositor::<Rgb<u8>>::composite(&SrcOut),
            Basic::SrcAtop => Compositor::<Rgb<u8>>::composite(&SrcAtop),
            Basic::DstOver => Compositor::<Rgb<u8>>::composite(&DstOver),
            Basic::DstIn => Compositor::<Rgb<u8>>::composite(&DstIn),
            Basic::DstOut => Compositor::<Rgb<u8>>::composite(&DstOut),
            Basic::DstAtop => Compositor::<Rgb<u8>>::composite(&DstAtop),
            Basic::Xor => Compositor::<Rgb<u8>>::composite(&Xor),
            Basic::Add => Compositor::<Rgb<u8>>::composite(&Add),
            Basic::Darken => Compositor::<Rgb<u8>>::composite(&Darken),
            Basic::Lighten => Compositor::<Rgb<u8>>::composite(&Lighten),
            Basic::Multiply => Compositor::<Rgb<u8>>::composite(&Multiply),
            Basic::Screen => Compositor::<Rgb<u8>>::composite(&Screen),
            Basic::Overlay => Compositor::<Rgb<u8>>::composite(&Overlay),
            Basic::HardLight => Compositor::<Rgb<u8>>::composite(&HardLight),
            Basic::Dodge => Compositor::<Rgb<u8>>::composite(&Dodge),
            Basic::Burn => Compositor::<Rgb<u8>>::composite(&Burn),
            Basic::SoftLight => Compositor::<Rgb<u8>>::composite(&SoftLight),
            Basic::Difference => Compositor::<Rgb<u8>>::composite(&Difference),
            Basic::Exclusion => Compositor::<Rgb<u8>>::composite(&Exclusion),
        }
    }

    fn composite_with_alpha(&self) -> Self::F1 {
        match self {
            Basic::Clear => Compositor::<Rgb<u8>>::composite_with_alpha(&Clear),
            Basic::Src => Compositor::<Rgb<u8>>::composite_with_alpha(&Src),
            Basic::Dst => Compositor::<Rgb<u8>>::composite_with_alpha(&Dst),
            Basic::SrcOver => Compositor::<Rgb<u8>>::composite_with_alpha(&SrcOver),
            Basic::SrcIn => Compositor::<Rgb<u8>>::composite_with_alpha(&SrcIn),
            Basic::SrcOut => Compositor::<Rgb<u8>>::composite_with_alpha(&SrcOut),
            Basic::SrcAtop => Compositor::<Rgb<u8>>::composite_with_alpha(&SrcAtop),
            Basic::DstOver => Compositor::<Rgb<u8>>::composite_with_alpha(&DstOver),
            Basic::DstIn => Compositor::<Rgb<u8>>::composite_with_alpha(&DstIn),
            Basic::DstOut => Compositor::<Rgb<u8>>::composite_with_alpha(&DstOut),
            Basic::DstAtop => Compositor::<Rgb<u8>>::composite_with_alpha(&DstAtop),
            Basic::Xor => Compositor::<Rgb<u8>>::composite_with_alpha(&Xor),
            Basic::Add => Compositor::<Rgb<u8>>::composite_with_alpha(&Add),
            Basic::Darken => Compositor::<Rgb<u8>>::composite_with_alpha(&Darken),
            Basic::Lighten => Compositor::<Rgb<u8>>::composite_with_alpha(&Lighten),
            Basic::Multiply => Compositor::<Rgb<u8>>::composite_with_alpha(&Multiply),
            Basic::Screen => Compositor::<Rgb<u8>>::composite_with_alpha(&Screen),
            Basic::Overlay => Compositor::<Rgb<u8>>::composite_with_alpha(&Overlay),
            Basic::HardLight => Compositor::<Rgb<u8>>::composite_with_alpha(&HardLight),
            Basic::Dodge => Compositor::<Rgb<u8>>::composite_with_alpha(&Dodge),
            Basic::Burn => Compositor::<Rgb<u8>>::composite_with_alpha(&Burn),
            Basic::SoftLight => Compositor::<Rgb<u8>>::composite_with_alpha(&SoftLight),
            Basic::Difference => Compositor::<Rgb<u8>>::composite_with_alpha(&Difference),
            Basic::Exclusion => Compositor::<Rgb<u8>>::composite_with_alpha(&Exclusion),
        }
    }
}
