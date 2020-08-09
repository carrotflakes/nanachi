use nanachi::{
    image::{ImageBuffer, Rgb},
    path2::{Path, PathAnchor},
    point::Point,
    position_color,
};

fn main() {
    let (width, height) = (512, 512);
    let mut img = ImageBuffer::from_pixel(width, height, Rgb([250u8, 250, 250]));

    {
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
        for p in paths[0].anchors.iter() {
            println!("{:?}", p);
        }
        let e: Vec<_> = paths.iter().flat_map(|p| p.edges()).collect();
        for p in e.iter() {
            match p {
                nanachi::path2::PathEdge::Line(p1, p2) => {if (p1.1 - p2.1).abs() < 0.1 {println!("!{:?}", p);};}
                nanachi::path2::PathEdge::Arc { center, radius, angle1, angle2 } => {();}
            }
            //println!("e: {:?}", p);
        }
        nanachi::fill_path2::draw_fill(
            &mut img,
            &e,
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
        let paths = path.edge_path(6.5);
        nanachi::fill_path2::draw_fill(
            &mut img,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &position_color::Constant::new(Rgb([40, 40, 250])),
        );
    }

    let res = img.save("./path.png");
    println!("save: {:?}", res);
}
