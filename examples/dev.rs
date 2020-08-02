use nanachi::{
    bezier::{Bezier2, Bezier3},
    k_curve,
    point::Point,
};

fn main() {
    {
        let b = Bezier3 {
            points: vec![
                Point(0.0, 0.0),
                Point(0.0, 1.0),
                Point(0.0, 1.0),
                Point(1.0, 0.0),
            ],
            close: false,
        };
        println!("{:?}", b.as_lines_points(8));
        let b3 = Bezier2 {
            points: vec![Point(0.0, 0.0), Point(0.5, 1.0), Point(1.0, 0.0)],
            close: false,
        };
        println!("{:?}", b3.as_lines_points(8));

        let b = k_curve::k_curve(
            vec![Point(0.0, 0.0), Point(10.0, 0.0), Point(0.0, 10.0)],
            false,
            3,
        );
        println!("{:?}", b);
        println!("{:?}", b.as_lines_points(4));
        let b = k_curve::k_curve(
            vec![Point(0.0, 0.0), Point(10.0, 0.0), Point(0.0, 10.0)],
            true,
            3,
        );
        println!("{:?}", b);
        println!("{:?}", b.as_lines_points(4));
    }
    let path2 = nanachi::k_curve::k_curve(vec![], false, 4).as_lines_points(10);
    println!("path2 {:?}", path2);
}
