pub trait Pixel: image::Pixel<Subpixel = u8> + 'static {}

impl Pixel for image::Rgb<u8> {}
impl Pixel for image::Rgba<u8> {}
