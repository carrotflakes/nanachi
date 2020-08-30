use nanachi::{
    image::{ImageBuffer, Rgb},
    path2::{Path, PathAnchor},
    point::Point,
    fill_color,
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
                PathAnchor::Point(Point(275. + x, 39. + y)),
            ],
            false,
        );
        let paths = path.edge_path(0.5);

        let pc = fill_color::Constant::new(Rgb([40, 40, 250]));
        nanachi::fill_path::draw_fill(
            img.width() as u32,
            img.height() as u32,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &mut nanachi::writer::alpha_blend(&mut img, &pc),
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
                PathAnchor::Point(Point(275. + x, 39. + y)),
            ],
            false,
        );
        let paths = path.edge_path(1.0);

        let pc = fill_color::Constant::new(Rgb([40, 40, 250]));
        nanachi::fill_path::draw_fill(
            img.width() as u32,
            img.height() as u32,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &mut nanachi::writer::alpha_blend(&mut img, &pc),
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
                PathAnchor::Point(Point(275. + x, 39. + y)),
            ],
            false,
        );
        let paths = path.edge_path(1.5);

        let pc = fill_color::Constant::new(Rgb([40, 40, 250]));
        nanachi::fill_path::draw_fill(
            img.width() as u32,
            img.height() as u32,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &mut nanachi::writer::alpha_blend(&mut img, &pc),
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
                PathAnchor::Point(Point(275. + x, 39. + y)),
            ],
            false,
        );
        let paths = path.edge_path(0.5);

        let pc = fill_color::Constant::new(Rgb([40, 40, 250]));
        nanachi::fill_path::draw_fill(
            img.width() as u32,
            img.height() as u32,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &mut nanachi::writer::alpha_blend(&mut img, &pc),
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
                PathAnchor::Point(Point(275. + x, 39. + y)),
            ],
            false,
        );
        let paths = path.edge_path(1.0);

        let pc = fill_color::Constant::new(Rgb([40, 40, 250]));
        nanachi::fill_path::draw_fill(
            img.width() as u32,
            img.height() as u32,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &mut nanachi::writer::alpha_blend(&mut img, &pc),
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
                PathAnchor::Point(Point(275. + x, 39. + y)),
            ],
            false,
        );
        let paths = path.edge_path(6.5);

        let pc = fill_color::Constant::new(Rgb([40, 40, 250]));
        nanachi::fill_path::draw_fill(
            img.width() as u32,
            img.height() as u32,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &mut nanachi::writer::alpha_blend(&mut img, &pc),
        );
    }

    let res = img.save("./path.png");
    println!("save: {:?}", res);
}
