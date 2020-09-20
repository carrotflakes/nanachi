use nanachi::{
    image::{ImageBuffer, Rgb},
    fill_color,
    matrix::Matrix2d,
    context::{Context, FillStyle},
    fill_rule,
    compositor::basic,
    path_data_notation,
};
use std::f64::consts::PI;

fn main() {
    let (width, height) = (512, 512);

    let mut img = ImageBuffer::from_pixel(width, height, Rgb([255u8, 255, 255]));
    let mut context = Context::new(&mut img).high_quality();

    let t = std::time::Instant::now();

    {
        let path = path_data_notation::parse("
        M 10,10 h 10
        m  0,10 h 10
        m  0,10 h 10
        M 40,20 h 10
        m  0,10 h 10
        m  0,10 h 10
        m  0,10 h 10
        M 50,50 h 10
        m-20,10 h 10
        m-20,10 h 10
        m-20,10 h 10").unwrap();
        context.transformed_context(&Matrix2d::new()
            .translate(0.0, 0.0)
        ).stroke(&path, &FillStyle {
            color: fill_color::Constant::new(Rgb([0, 0, 0])),
            fill_rule: fill_rule::NonZero,
            compositor: basic::SrcOver,
            pixel: Default::default(),
        }, 3.0);
    }

    {
        let path = path_data_notation::parse("
        M 10,10
        L 90,90
        V 10
        H 50").unwrap();
        context.transformed_context(&Matrix2d::new()
            .translate(100.0, 0.0)
        ).stroke(&path, &FillStyle {
            color: fill_color::Constant::new(Rgb([0, 0, 0])),
            fill_rule: fill_rule::NonZero,
            compositor: basic::SrcOver,
            pixel: Default::default(),
        }, 3.0);
    }

    {
        let path = path_data_notation::parse("
        M 110,10
        l 80,80
        v -80
        h -40").unwrap();
        context.transformed_context(&Matrix2d::new()
            .translate(100.0, 0.0)
        ).stroke(&path, &FillStyle {
            color: fill_color::Constant::new(Rgb([0, 0, 0])),
            fill_rule: fill_rule::NonZero,
            compositor: basic::SrcOver,
            pixel: Default::default(),
        }, 3.0);
    }

    {
        let path = path_data_notation::parse("
        M 10,90
        C 30,90 25,10 50,10
        S 70,90 90,90").unwrap();
        context.transformed_context(&Matrix2d::new()
            .translate(0.0, 100.0)
        ).stroke(&path, &FillStyle {
            color: fill_color::Constant::new(Rgb([0, 0, 0])),
            fill_rule: fill_rule::NonZero,
            compositor: basic::SrcOver,
            pixel: Default::default(),
        }, 3.0);
    }

    {
        let path = path_data_notation::parse("
        M 110,90
        c 20,0 15,-80 40,-80
        s 20,80 40,80").unwrap();
        context.transformed_context(&Matrix2d::new()
            .translate(0.0, 100.0)
        ).stroke(&path, &FillStyle {
            color: fill_color::Constant::new(Rgb([0, 0, 0])),
            fill_rule: fill_rule::NonZero,
            compositor: basic::SrcOver,
            pixel: Default::default(),
        }, 3.0);
    }

    {
        let path = path_data_notation::parse("
        M 10,50
        Q 25,25 40,50
        t 30,0 30,0 30,0 30,0 30,0").unwrap();
        context.transformed_context(&Matrix2d::new()
            .translate(0.0, 200.0)
        ).stroke(&path, &FillStyle {
            color: fill_color::Constant::new(Rgb([0, 0, 0])),
            fill_rule: fill_rule::NonZero,
            compositor: basic::SrcOver,
            pixel: Default::default(),
        }, 3.0);
    }

    dbg!(t.elapsed());

    let res = img.save("./path_data_notation.png");
    println!("save: {:?}", res);
}
