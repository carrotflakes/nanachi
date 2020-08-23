use nanachi::{
    affine::AugmentedMatrix,
    bezier::{Bezier2, Bezier3},
    draw, geometry,
    image::{ImageBuffer, Luma, Rgb},
    k_curve,
    path::Path,
    point::Point,
    position_color, primitives,
    models::{Arc, Ellipse, Quad},
};
use std::f64::consts::PI;

fn main() {
    let (width, height) = (512, 512);
    let mut img = ImageBuffer::from_pixel(width, height, Rgb([250u8, 250, 250]));

    draw::draw_hori_with_antialias(
        &mut img,
        (256, 256 + 200),
        PI * 0.1,
        &position_color::Constant::new(Rgb([200, 250, 250])),
    );
    draw::draw_hori_with_antialias(
        &mut img,
        (256 - 200, 256),
        PI * 0.6,
        &position_color::Constant::new(Rgb([200, 200, 250])),
    );
    draw::draw_hori_with_antialias(
        &mut img,
        (256, 256 - 200),
        PI * 1.1,
        &position_color::Constant::new(Rgb([250, 200, 250])),
    );
    draw::draw_hori_with_antialias(
        &mut img,
        (256 + 200, 256),
        PI * 1.6,
        &position_color::Constant::new(Rgb([250, 200, 200])),
    );

    draw::draw_line(&mut img, (10, 10), (100, 100), Rgb([250, 10, 10]));
    let mut path: Path = vec![
        (20f64, 20f64),
        (20., 50.),
        (50., 50.),
        (50., 20.),
        (20., 20.),
    ]
    .into_iter()
    .map(|x| x.into())
    .collect::<Vec<Point>>()
    .into();
    path.transform_mut(&AugmentedMatrix::new().rotate(-0.1));
    draw::draw_path(
        &mut img,
        &path.into() as &Vec<Point>,
        Rgb([10, 10, 250]),
        1.0,
    );

    let mut path = primitives::triangle(100.);
    path.transform_mut(&AugmentedMatrix::new().rotate(0.0).translate(100., 100.));
    let mut path2 = primitives::triangle(100.);
    path2.transform_mut(
        &AugmentedMatrix::new()
            .scale(0.5, 0.5)
            .rotate(0.0)
            .translate(100., 100.),
    );
    let pc = position_color::BlockCheck::new(Rgb([200, 200, 200]), Rgb([100, 100, 100]), 10.0);
    draw::draw_fill(
        &mut img,
        &vec![
            &path.clone().into() as &Vec<Point>,
            &path2.clone().into() as &Vec<Point>,
        ],
        &pc,
    );
    draw::draw_path(
        &mut img,
        &path.into() as &Vec<Point>,
        Rgb([10, 250, 10]),
        2.0,
    );

    let mut path = primitives::triangle(100.);
    path.transform_mut(&AugmentedMatrix::new().rotate(0.2).translate(100., 100.));
    draw::draw_path(
        &mut img,
        &path.into() as &Vec<Point>,
        Rgb([10, 250, 10]),
        2.0,
    );

    let mut path = primitives::triangle(100.);
    path.transform_mut(&AugmentedMatrix::new().rotate(0.4).translate(100., 100.));
    draw::draw_path(
        &mut img,
        &path.into() as &Vec<Point>,
        Rgb([10, 250, 10]),
        2.0,
    );

    let time = std::time::Instant::now();
    draw::draw_path(
        &mut img,
        &k_curve::k_curve(
            vec![
                Point(0.2, 0.2),
                Point(0.8, 0.2),
                Point(0.8, 0.8),
                Point(0.2, 0.8),
            ],
            true,
            0,
        )
        .as_lines_points(8)
        .iter()
        .map(|x| (x.0 * width as f64, x.1 * height as f64))
        .collect::<Vec<_>>(),
        Rgb([200, 0, 0]),
        2.0,
    );
    draw::draw_path(
        &mut img,
        &k_curve::k_curve(
            vec![
                Point(0.2, 0.2),
                Point(0.8, 0.2),
                Point(0.8, 0.8),
                Point(0.2, 0.8),
            ],
            true,
            1,
        )
        .as_lines_points(8)
        .iter()
        .map(|x| (x.0 * width as f64, x.1 * height as f64))
        .collect::<Vec<_>>(),
        Rgb([0, 200, 0]),
        2.0,
    );
    draw::draw_path(
        &mut img,
        &k_curve::k_curve(
            vec![
                Point(0.2, 0.2),
                Point(0.8, 0.2),
                Point(0.8, 0.8),
                Point(0.2, 0.8),
            ],
            true,
            2,
        )
        .as_lines_points(8)
        .iter()
        .map(|x| (x.0 * width as f64, x.1 * height as f64))
        .collect::<Vec<_>>(),
        Rgb([200, 200, 0]),
        2.0,
    );
    println!("{:?}", time.elapsed());

    {
        use nanachi::path2::{Path, PathAnchor};
        let path = Path::new(
            vec![
                PathAnchor::Point(Point(100.0, 200.0)),
                PathAnchor::Point(Point(150.0, 150.0)),
                PathAnchor::Point(Point(200.0, 200.0)),
                PathAnchor::Point(Point(300.0, 250.0)),
                PathAnchor::Arc(Arc {
                    center: Point(200.0, 300.0),
                    radius: 50.0,
                    angle1: 0.0,
                    angle2: 3.14,
                }),
                PathAnchor::Point(Point(180.0, 320.0)),
                PathAnchor::Point(Point(120.0, 340.0)),
            ],
            false,
        );
        let paths = path.edge_path(10.0);
        println!("{:?}", paths);
        println!(
            "!!!{:?}",
            paths.iter().flat_map(|p| p.edges()).collect::<Vec<_>>()
        );
        draw_fill(
            &mut img,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &position_color::Constant::new(Rgb([200, 100, 250])),
        );
        // for path in paths {
        //     nanachi::draw_path::draw_path2(&mut img, &path, Rgb([100, 0, 200]));
        // }
    }
    {
        let t = std::time::Instant::now();
        use nanachi::path2::{Path, PathAnchor};

        // 膨らんだ四角形
        let path = Path::new(
            vec![
                PathAnchor::Point(Point(100.0, 450.0)),
                PathAnchor::Point(Point(140.0, 460.0)),
                PathAnchor::Point(Point(160.0, 460.0)),
                PathAnchor::Point(Point(200.0, 450.0)),
                PathAnchor::Point(Point(210.0, 410.0)),
                PathAnchor::Point(Point(210.0, 390.0)),
                PathAnchor::Point(Point(200.0, 350.0)),
                PathAnchor::Point(Point(160.0, 340.0)),
                PathAnchor::Point(Point(140.0, 340.0)),
                PathAnchor::Point(Point(100.0, 350.0)),
                PathAnchor::Point(Point(90.0, 390.0)),
                PathAnchor::Point(Point(90.0, 410.0)),
            ],
            true,
        );

        draw_fill(
            &mut img,
            &path.edges(),
            &position_color::Constant::new(Rgb([200, 100, 20])),
        );

        // わずかに傾いた正方形
        let path = Path::new(
            vec![
                PathAnchor::Point(Point(300.5, 350.5)),
                PathAnchor::Point(Point(350.5, 351.5)),
                PathAnchor::Point(Point(349.5, 400.5)),
                PathAnchor::Point(Point(299.5, 399.5)),
            ]
            .into_iter()
            .rev()
            .collect(),
            true,
        );

        draw_fill(
            &mut img,
            &path.edges(),
            &position_color::Constant::new(Rgb([100, 200, 20])),
        );

        //欠けた円
        let path = Path::new(
            vec![
                PathAnchor::Arc(Arc {
                    // 方向(法線)に注意
                    center: Point(430.0, 430.0),
                    radius: 50.0,
                    angle1: PI * 0.1,
                    angle2: PI * 1.6,
                }), //.flip(),
            ],
            true,
        );
        let path2 = Path::new(
            vec![
                PathAnchor::Arc(Arc {
                    center: Point(430.0, 430.0),
                    radius: 25.0,
                    angle1: PI * 2.5,
                    angle2: PI * 1.0,
                }),
            ],
            true,
        );
        let mut e = path.edges();
        e.extend(path2.edges());
        println!("{:?}", path.edges());
        draw_fill(
            &mut img,
            &e,
            &position_color::Constant::new(Rgb([200, 100, 250])),
        );
        println!("fill_path elapsed: {:?}", t.elapsed());
    }

    {
        use nanachi::path2::{Path, PathAnchor};
        let path = Path::new(
            vec![
                PathAnchor::Ellipse(Ellipse {
                    center: Point(400.0, 100.0),
                    radius_x: 10.0,
                    radius_y: 80.0,
                    rotation: PI * 0.2,
                    angle1: PI * 0.6,
                    angle2: PI * 1.3,
                }),
            ],
            true,
        );
        // let se: nanachi::fill_path::SkewEllipse = Ellipse {
        //     center: Point(430.0, 100.0),
        //     radius_x: 50.0,
        //     radius_y: 10.0,
        //     rotation: 0.0,
        //     angle1: PI * 0.,
        //     angle2: PI * 2.0,
        // }.into();
        // dbg!(se);
        draw_fill(
            &mut img,
            &path.edges(),
            &position_color::Constant::new(Rgb([200, 200, 50])),
        );
    }

    {
        use nanachi::path2::{Path, PathAnchor};
        let paths = vec![Path::new(
            vec![
                PathAnchor::Quad(Quad {
                    start: Point(300.0, 60.0),
                    end: Point(250.0, 10.0),
                    control1: Point(300.0, 10.0),
                }),
                PathAnchor::Quad(Quad {
                    start: Point(250.0, 10.0),
                    end: Point(220.0, 50.0),
                    control1: Point(200.0, 10.0),
                }),
                PathAnchor::Quad(Quad {
                    start: Point(220.0, 50.0),
                    end: Point(300.0, 60.0),
                    control1: Point(260.0, 80.0),
                }),
            ],
            true,
        ), Path::new(
            vec![
                PathAnchor::Quad(Quad {
                    start: Point(250.0, 20.0),
                    end: Point(290.0, 60.0),
                    control1: Point(300.0, 10.0),
                }),
                PathAnchor::Quad(Quad {
                    start: Point(290.0, 60.0),
                    end: Point(230.0, 50.0),
                    control1: Point(260.0, 70.0),
                }),
                PathAnchor::Quad(Quad {
                    start: Point(230.0, 50.0),
                    end: Point(250.0, 20.0),
                    control1: Point(200.0, 10.0),
                }),
            ],
            true,
        )];

        // dbg!(nanachi::bezier_area::separate_quad_n(&Quad {
        //     start: Point(250.0, 20.0),
        //     end: Point(290.0, 60.0),
        //     control1: Point(300.0, 10.0),
        // }, 3));

        draw_fill(
            &mut img,
            //&path.edge_path(5.0)[0..1].iter().flat_map(|p| p.edges()).collect(),
            //&path.edges(),
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &position_color::Constant::new(Rgb([50, 250, 10])),
        );
    }

    let res = img.save("./symbols.png");
    println!("save: {:?}", res);
}

fn draw_fill<X, C: nanachi::position_color::PositionColor<X>>(
    img: &mut ImageBuffer<X, Vec<u8>>,
    edges: &Vec<nanachi::path2::PathEdge>,
    position_color: &C,
) where
    X: image::Pixel<Subpixel = u8> + 'static,
{
    nanachi::fill_path::draw_fill(
        img.width() as u32,
        img.height() as u32,
        edges,
        &mut nanachi::writer::alpha_blend(img, position_color, nanachi::writer::FillRule::NonZero),
    );
}
