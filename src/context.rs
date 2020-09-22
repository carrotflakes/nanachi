use crate::{
    compositor::Compositor,
    fill_color::{FillColor, Transform},
    fill_path::{draw_fill, draw_fill_no_aa},
    fill_rule::FillRule,
    matrix::Matrix2d,
    path::Path,
    path_flatten::path_flatten,
    path_outline::{path_outline, Cap, Join},
    path_transform::path_transform,
    pixel::Pixel,
    point::Point,
    writer::img_writer,
    buffer::Buffer,
};

pub struct FillStyle<P, FC, C, FR>
where
    P: Pixel,
    FC: FillColor<P>,
    C: Compositor<P> + 'static,
    FR: FillRule,
{
    pub color: FC,
    pub compositor: C,
    pub fill_rule: FR,
    pub pixel: std::marker::PhantomData<P>,
}

impl<P, FC, C, FR> FillStyle<P, FC, C, FR>
where
    P: Pixel,
    FC: FillColor<P>,
    C: Compositor<P> + 'static,
    FR: FillRule,
{
}

pub struct Context<'a, P: Pixel, B: Buffer<P>> {
    pub image: &'a mut B,
    pub flatten_tolerance: f64,
    pub antialiasing: bool,
    pub join: Join,
    pub cap: Cap,
    pub matrix: Matrix2d,
    pub pixel: std::marker::PhantomData<P>,
}

impl<'a, P, B> Context<'a, P, B>
where
    P: Pixel,
    B: Buffer<P>,
{
    pub fn from_image(image: &'a mut B) -> Context<'a, P, B> {
        Context {
            image,
            flatten_tolerance: 1.0,
            antialiasing: true,
            join: Join::Bevel,
            cap: Cap::Butt,
            matrix: Matrix2d::default(),
            pixel: Default::default(),
        }
    }

    pub fn low_quality(self) -> Context<'a, P, B> {
        Context {
            flatten_tolerance: 2.0,
            antialiasing: false,
            join: Join::Bevel,
            cap: Cap::Butt,
            ..self
        }
    }

    pub fn high_quality(self) -> Context<'a, P, B> {
        Context {
            flatten_tolerance: 0.1,
            antialiasing: true,
            join: Join::Round,
            cap: Cap::Round,
            ..self
        }
    }

    pub fn child<'b>(&'b mut self) -> Context<'b, P, B> {
        Context {
            image: self.image,
            flatten_tolerance: self.flatten_tolerance,
            antialiasing: self.antialiasing,
            join: self.join.clone(),
            cap: self.cap.clone(),
            matrix: self.matrix,
            pixel: self.pixel,
        }
    }

    pub fn transformed_context<'b>(&'b mut self, matrix: &Matrix2d) -> Context<'b, P, B> {
        Context {
            matrix: self.matrix.then(&matrix),
            ..self.child()
        }
    }

    pub fn fill<FC: FillColor<P>, C: Compositor<P>, FR: FillRule>(
        &mut self,
        path: &Path,
        fill_style: &FillStyle<P, FC, C, FR>,
    ) {
        let path = if self.matrix.is_unit() {
            path_flatten(path, self.flatten_tolerance)
        } else {
            let path = path_transform(path, &self.matrix);
            path_flatten(&path, self.flatten_tolerance)
        };
        self.fill_(fill_style, &path, self.antialiasing);
    }

    pub fn stroke<FC: FillColor<P>, C: Compositor<P>, FR: FillRule>(
        &mut self,
        path: &Path,
        fill_style: &FillStyle<P, FC, C, FR>,
        width: f64,
    ) {
        let path = if self.matrix.is_unit() {
            path_flatten(path, self.flatten_tolerance)
        } else {
            let path = path_transform(path, &self.matrix);
            path_flatten(&path, self.flatten_tolerance)
        };
        let path = path_outline(&path, width / 2.0, &self.join, &self.cap);
        self.fill_(fill_style, &path, self.antialiasing);
    }

    pub fn stroke_with_style<FC: FillColor<P>, C: Compositor<P>, FR: FillRule>(
        &mut self,
        path: &Path,
        fill_style: &FillStyle<P, FC, C, FR>,
        width: f64,
        join: &Join,
        cap: &Cap,
    ) {
        let path = if self.matrix.is_unit() {
            path_flatten(path, self.flatten_tolerance)
        } else {
            let path = path_transform(path, &self.matrix);
            path_flatten(&path, self.flatten_tolerance)
        };
        let path = path_outline(&path, width / 2.0, join, cap);
        self.fill_(fill_style, &path, self.antialiasing);
    }

    pub fn clear<FC: FillColor<P>>(&mut self, fill_color: &FC) {
        let (w, h) = self.image.dimensions();
        let inverted_matrix = self.matrix.inverse();
        for y in 0..h {
            for x in 0..w {
                let p = inverted_matrix.apply(Point(x as f64, y as f64));
                self.image.put_pixel(x, y, fill_color.fill_color(p.0, p.1));
            }
        }
    }

    #[inline]
    fn fill_<FC: FillColor<P>, C: Compositor<P>, FR: FillRule>(
        &mut self,
        fill_style: &FillStyle<P, FC, C, FR>,
        path: &Path,
        antialiasing: bool,
    ) {
        let (width, height) = self.image.dimensions();
        let color = Transform::new(&fill_style.color, self.matrix);
        let mut writer = img_writer(self.image, &color, &fill_style.compositor);
        if antialiasing {
            draw_fill(width, height, &path, fill_style.fill_rule, &mut writer, !fill_style.compositor.keep_dst_on_transparent_src());
        } else {
            draw_fill_no_aa(width, height, &path, fill_style.fill_rule, &mut writer);
        }
    }
}
