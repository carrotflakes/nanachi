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
    writer::img_writer,
};
use image::{ImageBuffer, Pixel};

pub struct FillStyle<P, FC, C, FR>
where
    P: Pixel<Subpixel = u8> + 'static,
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
    P: Pixel<Subpixel = u8> + 'static,
    FC: FillColor<P>,
    C: Compositor<P> + 'static,
    FR: FillRule,
{
}

pub struct Context<'a, P>
where
    P: Pixel<Subpixel = u8> + 'static,
{
    pub image: &'a mut ImageBuffer<P, Vec<u8>>,
    pub flatten_tolerance: f64,
    pub antialiasing: bool,
    pub join: Join,
    pub cap: Cap,
    pub matrix: Matrix2d,
}

impl<'a, P> Context<'a, P>
where
    P: Pixel<Subpixel = u8> + 'static,
{
    pub fn new(image: &'a mut ImageBuffer<P, Vec<u8>>) -> Context<'a, P> {
        Context {
            image,
            flatten_tolerance: 1.0,
            antialiasing: true,
            join: Join::Bevel,
            cap: Cap::Butt,
            matrix: Matrix2d::default(),
        }
    }

    pub fn low_quality(self) -> Context<'a, P> {
        Context {
            image: self.image,
            flatten_tolerance: 2.0,
            antialiasing: false,
            join: Join::Bevel,
            cap: Cap::Butt,
            matrix: self.matrix,
        }
    }

    pub fn high_quality(self) -> Context<'a, P> {
        Context {
            image: self.image,
            flatten_tolerance: 0.1,
            antialiasing: true,
            join: Join::Round,
            cap: Cap::Round,
            matrix: self.matrix,
        }
    }

    pub fn child<'b>(&'b mut self) -> Context<'b, P> {
        Context {
            image: self.image,
            flatten_tolerance: self.flatten_tolerance,
            antialiasing: self.antialiasing,
            join: self.join.clone(),
            cap: self.cap.clone(),
            matrix: self.matrix,
        }
    }

    pub fn transformed_context<'b>(&'b mut self, matrix: &Matrix2d) -> Context<'b, P> {
        Context {
            image: self.image,
            flatten_tolerance: self.flatten_tolerance,
            antialiasing: self.antialiasing,
            join: self.join.clone(),
            cap: self.cap.clone(),
            matrix: self.matrix.then(&matrix),
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

    #[inline]
    fn fill_<FC: FillColor<P>, C: Compositor<P>, FR: FillRule>(
        &mut self,
        fill_style: &FillStyle<P, FC, C, FR>,
        path: &Path,
        antialiasing: bool,
    ) {
        let width = self.image.width();
        let height = self.image.height();
        let color = Transform::new(&fill_style.color, self.matrix);
        let mut writer = img_writer(self.image, &color, &fill_style.compositor);
        if antialiasing {
            draw_fill(width, height, &path, fill_style.fill_rule, &mut writer);
        } else {
            draw_fill_no_aa(width, height, &path, fill_style.fill_rule, &mut writer);
        }
    }
}
