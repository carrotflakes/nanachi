use crate::point::Point;

pub struct Bezier3 {
    pub points: Vec<Point>,
    pub close: bool,
}

impl Bezier3 {
    pub fn get_point(&self, pos: f64) -> Point {
        let i = pos.floor() as usize * 3;
        let ps = &self.points[i..i + 4];
        let v = pos.fract();
        let iv = 1.0 - v;
        (ps[0] + ps[1] * v) * iv + (ps[3] + ps[2] * iv) * v
    }

    pub fn as_lines_points(&self, division: usize) -> Vec<Point> {
        assert!(1 <= division);
        if self.points.is_empty() {
            return Vec::new();
        }
        let mut vec = Vec::new();
        for i in 0..self.anchor_num() - 1 {
            vec.push(self.points[i * 3]);
            for j in 1..division {
                vec.push(self.get_point(i as f64 + j as f64 / division as f64));
            }
        }
        vec.push(*self.points.last().unwrap());
        vec
    }

    pub fn anchor_num(&self) -> usize {
        self.points.len() / 3 + 1
    }
}

#[derive(Debug)]
pub struct Bezier2 {
    pub points: Vec<Point>,
    pub close: bool,
}

impl Bezier2 {
    pub fn get_point(&self, pos: f64) -> Point {
        let i = pos.floor() as usize * 2;
        let ps = &self.points[i..i + 3];
        let v = pos.fract();
        let iv = 1.0 - v;
        ps[0] * iv.powi(2) + ps[1] * v * iv * 2.0 + ps[2] * v.powi(2)
    }

    pub fn as_lines_points(&self, division: usize) -> Vec<Point> {
        assert!(1 <= division);
        if self.points.is_empty() {
            return Vec::new();
        }
        let mut vec = Vec::new();
        for i in 0..self.anchor_num() - 1 {
            vec.push(self.points[i * 2]);
            for j in 1..division {
                vec.push(self.get_point(i as f64 + j as f64 / division as f64));
            }
        }
        vec.push(*self.points.last().unwrap());
        vec
    }

    pub fn anchor_num(&self) -> usize {
        self.points.len() / 2 + 1
    }
}
