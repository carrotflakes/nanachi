//! Κ-curves implementation

use crate::point::Point;

struct K {
    points: Vec<Point>,
    close: bool,
    n: usize,
    l: Vec<f32>,
    b: Vec<Point>,
    t: Vec<f32>,
    a: Vec<f32>,
}

impl K {
    pub fn new(points: Vec<Point>, close: bool) -> Self {
        let n = points.len();
        assert!(2 <= n);

        let mut l = vec![0.5; n];
        if !close {
            l[0] = 0.0;
            l[n - 2] = 1.0;
        }

        let mut b = Vec::with_capacity(n * 2);
        for i in 0..n {
            let j = (i + n - 1) % n;
            b.push(points[j].lerp(points[i], l[i]));
            b.push(points[i]);
        }

        let mut a = vec![0.0; (n + 2) * 3];
        let r = [0.0, 1.0, 0.0];
        a[0..3].copy_from_slice(&r);
        a[(n + 1) * 3..(n + 2) * 3].copy_from_slice(&r);
        if !close {
            a[3..6].copy_from_slice(&r);
            a[n * 3..(n + 1) * 3].copy_from_slice(&r);
        }

        K {
            points,
            close,
            n,
            l,
            b,
            t: vec![0.0; n],
            a,
        }
    }

    pub fn optimize(&mut self, iteration: usize) {
        for _ in 0..iteration {
            self.step1();
            self.step2();
            self.step3();
            self.step4();
        }
    }

    fn step1(&mut self) {
        let n = self.n;
        let b = &self.b;
        for i in if self.close { 0..n } else { 1..n - 2 } {
            let j = (i + 1) % n;
            let t1 = tri_area(b[i * 2], b[i * 2 + 1], b[j * 2 + 1]);
            let t2 = tri_area(b[i * 2 + 1], b[j * 2 + 1], b[(j + 1) % n * 2]);
            self.l[i] = if (t1 - t2).abs() < 0.00001 {
                0.5
            } else {
                (t1 - (t1 * t2).sqrt()) / (t1 - t2)
            };
        }
    }

    fn step2(&mut self) {
        let n = self.n;
        let b = &mut self.b;
        for i in 0..n {
            let j = (i + 1) % n;
            b[j * 2] = b[i * 2 + 1].lerp(b[j * 2 + 1], self.l[i]);
        }
    }

    fn step3(&mut self) {
        let ps = &self.points;
        let n = self.n;
        let b = &self.b;
        let t = &mut self.t;
        for i in 0..n {
            let j = (i + 1) % n;
            let c2 = b[j * 2] - b[i * 2];
            let p = ps[i] - b[i * 2];
            t[i] = if c2 == Point::from((0.0, 0.0)) && p == Point::from((0.0, 0.0)) {
                0.5
            } else {
                let a = c2.dot(&c2);
                let b = -3.0 * c2.dot(&p);
                let c = (p * 2.0 + c2).dot(&p);
                let d = -p.dot(&p);
                if a == 0.0 && b == 0.0 {
                    -d / c
                } else {
                    solve_cubic_equation_with_check(a, b, c, d)
                }
            };
        }
    }

    fn step4(&mut self) {
        let n = self.n;
        let a = &mut self.a;
        let t = &mut self.t;
        let l = &self.l;
        for i in if self.close { 0..n } else { 1..n - 1 } {
            let prev = (i + n - 1) % n;
            let next = (i + 1) % n;

            //ランクが下がってしまう場合微調整
            if t[i] == 1.0 && t[next] == 0.0 || !self.close && i == n - 2 && t[i] == 1.0 {
                t[i] = 0.99999;
            }
            if !self.close && i == 1 && t[i] == 0.0 {
                t[i] = 0.00001;
            }

            let tmp = (1.0 - t[i]).powi(2);
            a[(i + 1) * 3] = (1.0 - l[prev]) * tmp;
            a[(i + 1) * 3 + 1] = l[prev] * tmp + (2.0 - (1.0 + l[i]) * t[i]) * t[i];
            a[(i + 1) * 3 + 2] = l[i] * t[i].powi(2);
        }
        solve_tridiagonal_equation(a, &mut self.b, &self.points);
    }

    pub fn get_bezier_points(&self) -> Vec<Point> {
        let mut points = self.b.clone();
        if self.close {
            points.push(points[0]);
        } else {
            if self.n <= 2 {
                points[0] = self.points[0];
                points[1] = (points[0] + points[2]) / 2.0; // workaround!
                points.pop();
            } else {
                points = Vec::from(&points[2..self.n * 2 - 1])
            };
        }
        points
    }
}

pub fn k_curve(points: Vec<Point>, close: bool, iteration: usize) -> Vec<Point> {
    if points.len() <= 1 {
        return Vec::new();
    }
    let mut k = K::new(points, close);
    k.optimize(iteration);
    k.get_bezier_points()
}

fn tri_area(p1: Point, p2: Point, p3: Point) -> f32 {
    ((p1.x() - p3.x()) * (p2.y() - p3.y()) - (p2.x() - p3.x()) * (p1.y() - p3.y())).abs() / 2.0
}

fn solve_cubic_equation(a: f32, b: f32, c: f32, d: f32) -> f32 {
    let b = b / (a * 3.0);
    let c = c / a;
    let d = d / a;
    let p = c / 3.0 - b.powi(2);
    let q = b.powi(3) - (b * c - d) / 2.0;
    let dd = q.powi(2) + p.powi(3);

    if dd.abs() < 1.0e-12 {
        let r = q.cbrt() - b;
        return 1.0f32.min(if 0.0 <= r { r } else { r * -2.0 });
    }
    if dd > 0.0 {
        let sqrtdd = dd.sqrt();
        let r = (-q + sqrtdd).cbrt() + (-q - sqrtdd).cbrt() - b;
        return r.clamp(0.0, 1.0);
    }
    let tmp = 2.0 * (-p).sqrt();
    let arg = (-dd).sqrt().atan2(-q) / 3.0;
    let pi2d3 = 2.0 * std::f32::consts::PI / 3.0;
    let r1 = tmp * arg.cos() - b;
    if 0.0 <= r1 && r1 <= 1.0 {
        return r1;
    }
    let r2 = tmp * (arg + pi2d3).cos() - b;
    if 0.0 <= r2 && r2 <= 1.0 {
        return r2;
    }
    let r3 = tmp * (arg - pi2d3 * 2.0).cos() - b;
    if 0.0 <= r3 && r3 <= 1.0 {
        return r3;
    }
    panic!("Invalid solution: {}, {}, {}", r1, r2, r3)
}

fn solve_cubic_equation_with_check(a: f32, b: f32, c: f32, d: f32) -> f32 {
    let x = solve_cubic_equation(a, b, c, d);
    if !((a * x.powi(3) + b * x.powi(2) + c * x + d).abs() < 0.0001) {
        println!(
            "{} {} {} {} => {}",
            a,
            b,
            c,
            d,
            a * x.powi(3) + b * x.powi(2) + c * x + d
        );
        assert!((a * x.powi(3) + b * x.powi(2) + c * x + d).abs() < 0.0001);
    }
    x
}

fn solve_tridiagonal_equation(a: &mut Vec<f32>, b: &mut Vec<Point>, ps: &Vec<Point>) {
    let n = ps.len();
    for i in 0..n + 1 {
        a[(i + 1) * 3] /= a[i * 3 + 1];
        a[(i + 1) * 3 + 1] -= a[(i + 1) * 3] * a[i * 3 + 2];
    }

    b[(n - 1) * 2 + 1] = ps[n - 1];
    for i in 0..n {
        b[i * 2 + 1] = ps[i] - b[(i + n - 1) % n * 2 + 1] * a[(i + 1) * 3];
    }

    b[1] = b[1] / a[(n + 1) * 3 + 1];
    for i in (0..=n).rev() {
        let j = i % n;
        let ii = (i + n - 1) % n;
        b[ii * 2 + 1] = (b[ii * 2 + 1] - b[j * 2 + 1] * a[i * 3 + 2]) / a[i * 3 + 1];
    }
}
