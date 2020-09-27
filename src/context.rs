use crate::{
    buffer::{Buffer, GenericBuffer},
    compositor::Compositor,
    fill_color::{FillColor, Transform},
    fill_rule::FillRule,
    matrix::Matrix2d,
    path::Path,
    path_flatten::{path_flatten, path_flatten_only_cubic},
    path_outline::{path_outline, Cap, Join},
    path_transform::path_transform,
    pixel::Pixel,
    point::Point,
    rasterizer::Rasterizer,
    writer::img_writer,
};
use std::borrow::BorrowMut;

pub type ChildContext<'a, P, B> = Context<P, B, &'a mut B, &'a mut Rasterizer>;

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

pub struct Context<P, B, I, R>
where
    P: Pixel,
    B: Buffer<P>,
    I: BorrowMut<B>,
    R: BorrowMut<Rasterizer>,
{
    pub image: I,
    pub flatten: bool,
    pub flatten_tolerance: f64,
    pub antialiasing: bool,
    pub join: Join,
    pub cap: Cap,
    pub matrix: Matrix2d,
    rasterizer: R,
    pixel: std::marker::PhantomData<P>,
    b: std::marker::PhantomData<B>,
}

impl<P, B, I, R> Context<P, B, I, R>
where
    P: Pixel,
    B: Buffer<P>,
    I: BorrowMut<B>,
    R: BorrowMut<Rasterizer>,
{
    pub fn low_quality(self) -> Context<P, B, I, R> {
        Context {
            flatten: true,
            flatten_tolerance: 2.0,
            antialiasing: false,
            join: Join::Bevel,
            cap: Cap::Butt,
            ..self
        }
    }

    pub fn high_quality(self) -> Context<P, B, I, R> {
        Context {
            flatten: false,
            flatten_tolerance: 0.1,
            antialiasing: true,
            join: Join::Round,
            cap: Cap::Round,
            ..self
        }
    }

    pub fn fill<FC: FillColor<P>, C: Compositor<P>, FR: FillRule>(
        &mut self,
        path: &Path,
        fill_style: &FillStyle<P, FC, C, FR>,
    ) {
        let path = self.path_transform_and_flatten(path);
        self.fill_(fill_style, &path, self.antialiasing);
    }

    pub fn stroke<FC: FillColor<P>, C: Compositor<P>, FR: FillRule>(
        &mut self,
        path: &Path,
        fill_style: &FillStyle<P, FC, C, FR>,
        width: f64,
    ) {
        let path = self.path_transform_and_flatten(path);
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
        let path = self.path_transform_and_flatten(path);
        let path = path_outline(&path, width / 2.0, join, cap);
        self.fill_(fill_style, &path, self.antialiasing);
    }

    pub fn path_transform_and_flatten(&self, path: &Path) -> Path {
        if self.matrix.is_unit() {
            if self.flatten {
                path_flatten(path, self.flatten_tolerance)
            } else {
                path_flatten_only_cubic(path, self.flatten_tolerance)
            }
        } else {
            let path = path_transform(path, &self.matrix);
            if self.flatten {
                path_flatten(&path, self.flatten_tolerance)
            } else {
                path_flatten_only_cubic(&path, self.flatten_tolerance)
            }
        }
    }

    pub fn clear<FC: FillColor<P>>(&mut self, fill_color: &FC) {
        let image = self.image.borrow_mut();
        let (w, h) = image.dimensions();
        let inverted_matrix = self.matrix.inverse();
        for y in 0..h {
            for x in 0..w {
                let p = inverted_matrix.apply(Point(x as f64, y as f64));
                image.put_pixel(x, y, fill_color.fill_color(p.0, p.1));
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
        let color = Transform::new(&fill_style.color, self.matrix);
        let mut writer = img_writer(self.image.borrow_mut(), &color, &fill_style.compositor);
        let pis = crate::path_flatten::Flatten::new(path.0.iter(), self.flatten_tolerance);
        if antialiasing {
            self.rasterizer.borrow_mut().rasterize(
                pis,
                fill_style.fill_rule,
                &mut writer,
                !fill_style.compositor.keep_dst_on_transparent_src(),
            );
        } else {
            self.rasterizer.borrow_mut().rasterize_no_aa(
                pis,
                fill_style.fill_rule,
                &mut writer,
                !fill_style.compositor.keep_dst_on_transparent_src(),
            );
        }
    }
}

impl<'a, P> Context<P, GenericBuffer<P>, GenericBuffer<P>, Rasterizer>
where
    P: Pixel,
{
    pub fn new(width: u32, height: u32, pixel: P) -> Self {
        Context {
            image: GenericBuffer::new(width, height, pixel),
            rasterizer: Rasterizer::new(width, height),
            flatten: true,
            flatten_tolerance: 1.0,
            antialiasing: true,
            join: Join::Bevel,
            cap: Cap::Butt,
            matrix: Matrix2d::default(),
            pixel: Default::default(),
            b: Default::default(),
        }
    }
}

impl<'a, P, B> Context<P, B, &'a mut B, Rasterizer>
where
    P: Pixel,
    B: Buffer<P>,
{
    pub fn from_image(image: &'a mut B) -> Self {
        let (width, height) = image.dimensions();
        Context {
            image,
            rasterizer: Rasterizer::new(width, height),
            flatten: true,
            flatten_tolerance: 1.0,
            antialiasing: true,
            join: Join::Bevel,
            cap: Cap::Butt,
            matrix: Matrix2d::default(),
            pixel: Default::default(),
            b: Default::default(),
        }
    }
}

impl<'a, P, B, I, R> Context<P, B, I, R>
where
    P: Pixel,
    B: Buffer<P>,
    I: BorrowMut<B>,
    R: BorrowMut<Rasterizer>,
{
    pub fn child<'b>(&'b mut self) -> ChildContext<'b, P, B> {
        Context {
            image: self.image.borrow_mut(),
            rasterizer: self.rasterizer.borrow_mut(),
            flatten: self.flatten,
            flatten_tolerance: self.flatten_tolerance,
            antialiasing: self.antialiasing,
            join: self.join.clone(),
            cap: self.cap.clone(),
            matrix: self.matrix,
            pixel: self.pixel,
            b: Default::default(),
        }
    }

    pub fn transformed_context<'b>(&'b mut self, matrix: &Matrix2d) -> ChildContext<'b, P, B> {
        Context {
            matrix: self.matrix.then(&matrix),
            ..self.child()
        }
    }
}
