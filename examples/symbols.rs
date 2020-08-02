use nanachi::{
    affine::AugmentedMatrix,
    bezier::{Bezier2, Bezier3},
    draw, geometry,
    image::{ImageBuffer, Rgb},
    k_curve,
    path::Path,
    point::Point,
    primitives,
};

fn main() {
    let (width, height) = (512, 512);
    let mut img = ImageBuffer::from_pixel(width, height, Rgb([250u8, 250, 250]));

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
    path.transform_mut(&AugmentedMatrix::new().rotate(1.0));
    draw::draw_path(&mut img, &path.into() as &Vec<Point>, Rgb([10, 10, 250]));

    let mut path = primitives::triangle(100.);
    path.transform_mut(&AugmentedMatrix::new().rotate(0.0).translate(100., 100.));
    draw::draw_fill(&mut img, &path.clone().into() as &Vec<Point>, Rgb([100, 250, 100]));
    draw::draw_path(&mut img, &path.into() as &Vec<Point>, Rgb([10, 250, 10]));

    let mut path = primitives::triangle(100.);
    path.transform_mut(&AugmentedMatrix::new().rotate(0.2).translate(100., 100.));
    draw::draw_path(&mut img, &path.into() as &Vec<Point>, Rgb([10, 250, 10]));

    let mut path = primitives::triangle(100.);
    path.transform_mut(&AugmentedMatrix::new().rotate(0.4).translate(100., 100.));
    draw::draw_path(&mut img, &path.into() as &Vec<Point>, Rgb([10, 250, 10]));

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
    );

    let res = img.save("./symbols.png");
    println!("save: {:?}", res);
}
