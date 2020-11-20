//! `Context` provides high level API.

use crate::{
    buffer::{Buffer, GenericBuffer},
    compositor::Compositor,
    fill_color::{FillColor, Transform},
    fill_rule::FillRule,
    matrix::Matrix,
    path::Path,
    path_flatten::Flatten,
    path_outline::{path_outline, Cap, Join},
    path_segments::Segments,
    path_transform::path_transform,
    pixel::Pixel,
    point::Point,
    rasterize::RasterizeBuffer,
    writer::image_writer,
};
use std::borrow::BorrowMut;

pub type ChildContext<'a, P, B> = Context<P, B, &'a mut B, &'a mut RasterizeBuffer>;

#[derive(Clone)]
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
    pub fn new(color: FC, compositor: C, fill_rule: FR) -> Self {
        FillStyle {
            color,
            compositor,
            fill_rule,
            pixel: Default::default(),
        }
    }
}

pub struct Context<P, B, I, R>
where
    P: Pixel,
    B: Buffer<P>,
    I: BorrowMut<B>,
    R: BorrowMut<RasterizeBuffer>,
{
    pub image: I,
    pub flatten_tolerance: f64,
    pub antialiasing: bool,
    pub join: Join,
    pub cap: Cap,
    pub matrix: Matrix,
    rasterizer: R,
    pixel: std::marker::PhantomData<P>,
    b: std::marker::PhantomData<B>,
}

impl<P, B, I, R> Context<P, B, I, R>
where
    P: Pixel,
    B: Buffer<P>,
    I: BorrowMut<B>,
    R: BorrowMut<RasterizeBuffer>,
{
    /// Set to low quality.
    pub fn low_quality(self) -> Context<P, B, I, R> {
        Context {
            flatten_tolerance: 2.0,
            antialiasing: false,
            join: Join::Bevel,
            cap: Cap::Butt,
            ..self
        }
    }

    /// Set to high quality.
    pub fn high_quality(self) -> Context<P, B, I, R> {
        Context {
            flatten_tolerance: 0.1,
            antialiasing: true,
            join: Join::Round,
            cap: Cap::Round,
            ..self
        }
    }

    /// Fill the path with specified [`FillStyle`].
    pub fn fill<FC: FillColor<P>, C: Compositor<P>, FR: FillRule>(
        &mut self,
        path: &Path,
        fill_style: &FillStyle<P, FC, C, FR>,
    ) {
        let path = self.path_transform_and_flatten(path);
        self.fill_(fill_style, &path);
    }

    /// Draw stroke of the path with specified [`FillStyle`].
    pub fn stroke<FC: FillColor<P>, C: Compositor<P>, FR: FillRule>(
        &mut self,
        path: &Path,
        fill_style: &FillStyle<P, FC, C, FR>,
        width: f64,
    ) {
        let path = self.path_transform_and_flatten(path);
        let path = path_outline(&path, width / 2.0, &self.join, &self.cap);
        self.fill_(fill_style, &path);
    }

    /// Draw stroke of the path with specified [`FillStyle`], [`Join`] and [`Cap`].
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
        self.fill_(fill_style, &path);
    }

    fn path_transform_and_flatten(&self, path: &Path) -> Path {
        if self.matrix.is_unit() {
            Path::new(Flatten::new(path.0.iter(), self.flatten_tolerance).collect())
        } else {
            let path = path_transform(path, &self.matrix);
            Path::new(Flatten::new(path.0.iter(), self.flatten_tolerance).collect())
        }
    }

    /// Clear buffer entirely with specified [`FillColor`]
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
    ) {
        let color = Transform::new(&fill_style.color, self.matrix);
        let mut writer = image_writer(self.image.borrow_mut(), &color, &fill_style.compositor);
        let pis = Flatten::new(path.0.iter(), self.flatten_tolerance);
        let segments = Segments::new(pis);
        let write_transparent_src = !fill_style.compositor.keep_dst_on_transparent_src()
            || fill_style.fill_rule.is_inverse();
        if self.antialiasing {
            self.rasterizer.borrow_mut().rasterize(
                segments,
                fill_style.fill_rule,
                &mut writer,
                write_transparent_src,
            );
        } else {
            self.rasterizer.borrow_mut().rasterize_no_aa(
                segments,
                fill_style.fill_rule,
                &mut writer,
                write_transparent_src,
            );
        }
    }
}

impl<'a, P> Context<P, GenericBuffer<P>, GenericBuffer<P>, RasterizeBuffer>
where
    P: Pixel,
{
    /// Create [`Context`] with a [`GenericBuffer`].
    pub fn from_pixel(width: u32, height: u32, pixel: P) -> Self {
        Context {
            image: GenericBuffer::from_pixel(width, height, pixel),
            rasterizer: RasterizeBuffer::new(width, height),
            flatten_tolerance: 1.0,
            antialiasing: true,
            join: Join::Bevel,
            cap: Cap::Butt,
            matrix: Matrix::default(),
            pixel: Default::default(),
            b: Default::default(),
        }
    }
}

impl<'a, P, B> Context<P, B, &'a mut B, RasterizeBuffer>
where
    P: Pixel,
    B: Buffer<P>,
{
    /// Create [`Context`] from the [`Buffer`].
    pub fn from_image(image: &'a mut B) -> Self {
        let (width, height) = image.dimensions();
        Context {
            image,
            rasterizer: RasterizeBuffer::new(width, height),
            flatten_tolerance: 1.0,
            antialiasing: true,
            join: Join::Bevel,
            cap: Cap::Butt,
            matrix: Matrix::default(),
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
    R: BorrowMut<RasterizeBuffer>,
{
    /// Create child [`Context`].
    pub fn child<'b>(&'b mut self) -> ChildContext<'b, P, B> {
        Context {
            image: self.image.borrow_mut(),
            rasterizer: self.rasterizer.borrow_mut(),
            flatten_tolerance: self.flatten_tolerance,
            antialiasing: self.antialiasing,
            join: self.join.clone(),
            cap: self.cap.clone(),
            matrix: self.matrix,
            pixel: self.pixel,
            b: Default::default(),
        }
    }

    /// Create child [`Context`] and transform.
    pub fn transformed_context<'b>(&'b mut self, matrix: &Matrix) -> ChildContext<'b, P, B> {
        Context {
            matrix: matrix.then(&self.matrix),
            ..self.child()
        }
    }
}
