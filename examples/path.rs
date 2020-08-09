use nanachi::{
    affine::AugmentedMatrix,
    bezier::{Bezier2, Bezier3},
    draw, geometry,
    image::{ImageBuffer, Luma, Rgb},
    k_curve,
    path::Path,
    point::Point,
    position_color, primitives,
};
use std::f64::consts::PI;

fn main() {
    let (width, height) = (512, 512);
    let mut img = ImageBuffer::from_pixel(width, height, Rgb([250u8, 250, 250]));

    {
        use nanachi::path2::{Path, PathAnchor};
        let x = -100.5;
        let y = 0.5;
        let path = Path::new(
            vec![
                PathAnchor::Point(Point(200. + x, 40. + y)),
                PathAnchor::Point(Point(200. + x, 70. + y)),
                PathAnchor::Point(Point(240. + x, 70. + y)),
                PathAnchor::Point(Point(250. + x, 100. + y)),
                PathAnchor::Point(Point(200. + x, 100. + y)),
                PathAnchor::Point(Point(270. + x, 140. + y)),
                PathAnchor::Point(Point(280. + x, 40. + y)),
            ],
            false,
        );
        let paths = path.edge_path(0.5);
        nanachi::fill_path2::draw_fill(
            &mut img,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &position_color::Constant::new(Rgb([40, 40, 250])),
        );

        let x = -100.5;
        let y = 150.5;
        let path = Path::new(
            vec![
                PathAnchor::Point(Point(200. + x, 40. + y)),
                PathAnchor::Point(Point(200. + x, 70. + y)),
                PathAnchor::Point(Point(240. + x, 70. + y)),
                PathAnchor::Point(Point(250. + x, 100. + y)),
                PathAnchor::Point(Point(200. + x, 100. + y)),
                PathAnchor::Point(Point(270. + x, 140. + y)),
                PathAnchor::Point(Point(280. + x, 40. + y)),
            ],
            false,
        );
        let paths = path.edge_path(1.0);
        nanachi::fill_path2::draw_fill(
            &mut img,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &position_color::Constant::new(Rgb([40, 40, 250])),
        );

        let x = -100.5;
        let y = 300.5;
        let path = Path::new(
            vec![
                PathAnchor::Point(Point(200. + x, 40. + y)),
                PathAnchor::Point(Point(200. + x, 70. + y)),
                PathAnchor::Point(Point(240. + x, 70. + y)),
                PathAnchor::Point(Point(250. + x, 100. + y)),
                PathAnchor::Point(Point(200. + x, 100. + y)),
                PathAnchor::Point(Point(270. + x, 140. + y)),
                PathAnchor::Point(Point(280. + x, 40. + y)),
            ],
            false,
        );
        let paths = path.edge_path(1.5);
        nanachi::fill_path2::draw_fill(
            &mut img,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &position_color::Constant::new(Rgb([40, 40, 250])),
        );

        let x = 100.0;
        let y = 40.0;
        let path = Path::new(
            vec![
                PathAnchor::Point(Point(200. + x, 40. + y)),
                PathAnchor::Point(Point(200. + x, 70. + y)),
                PathAnchor::Point(Point(240. + x, 70. + y)),
                PathAnchor::Point(Point(250. + x, 100. + y)),
                PathAnchor::Point(Point(200. + x, 100. + y)),
                PathAnchor::Point(Point(270. + x, 140. + y)),
                PathAnchor::Point(Point(280. + x, 40. + y)),
            ],
            false,
        );
        let paths = path.edge_path(0.5);
        nanachi::fill_path2::draw_fill(
            &mut img,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &position_color::Constant::new(Rgb([40, 40, 250])),
        );

        let x = 100.0;
        let y = 190.0;
        let path = Path::new(
            vec![
                PathAnchor::Point(Point(200. + x, 40. + y)),
                PathAnchor::Point(Point(200. + x, 70. + y)),
                PathAnchor::Point(Point(240. + x, 70. + y)),
                PathAnchor::Point(Point(250. + x, 100. + y)),
                PathAnchor::Point(Point(200. + x, 100. + y)),
                PathAnchor::Point(Point(270. + x, 140. + y)),
                PathAnchor::Point(Point(280. + x, 40. + y)),
            ],
            false,
        );
        let paths = path.edge_path(1.0);
        nanachi::fill_path2::draw_fill(
            &mut img,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &position_color::Constant::new(Rgb([40, 40, 250])),
        );

        let x = 100.0;
        let y = 340.0;
        let path = Path::new(
            vec![
                PathAnchor::Point(Point(200. + x, 40. + y)),
                PathAnchor::Point(Point(200. + x, 70. + y)),
                PathAnchor::Point(Point(240. + x, 70. + y)),
                PathAnchor::Point(Point(250. + x, 100. + y)),
                PathAnchor::Point(Point(200. + x, 100. + y)),
                PathAnchor::Point(Point(270. + x, 140. + y)),
                PathAnchor::Point(Point(280. + x, 40. + y)),
            ],
            false,
        );
        let paths = path.edge_path(1.5);
        nanachi::fill_path2::draw_fill(
            &mut img,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &position_color::Constant::new(Rgb([40, 40, 250])),
        );
    }

    let res = img.save("./symbols.png");
    println!("save: {:?}", res);
}
