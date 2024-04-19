mod tokenize;

use crate::path::Path;
use crate::path_builder::PathBuilder;
use crate::point::Point;
use std::{iter::Peekable, str::Chars};
use tokenize::{Token, Tokenize};

enum LastControlPoint {
    Quad(Point),
    Cubic(Point),
    None,
}

/// Parse [SVG path notation](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths).
pub fn parse(str: &str) -> Result<Path, String> {
    let mut chars = str.chars();
    let mut tokens = Tokenize::new(&mut chars).peekable();
    let mut pb = PathBuilder::new();
    let mut last_control_point = LastControlPoint::None;
    loop {
        match tokens.next().unwrap()? {
            Token::LargeM => {
                let ns = parse_n_nums(&mut tokens, 2)?;
                pb.move_to(ns[0], ns[1]);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 2)?;
                    pb.line_to(ns[0], ns[1]);
                }
                last_control_point = LastControlPoint::None;
            }
            Token::SmallM => {
                let ns = parse_n_nums(&mut tokens, 2)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.move_to(current.0 + ns[0], current.1 + ns[1]);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 2)?;
                    let current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.line_to(current.0 + ns[0], current.1 + ns[1]);
                }
                last_control_point = LastControlPoint::None;
            }
            Token::LargeL => {
                let ns = parse_n_nums(&mut tokens, 2)?;
                pb.line_to(ns[0], ns[1]);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 2)?;
                    pb.line_to(ns[0], ns[1]);
                }
                last_control_point = LastControlPoint::None;
            }
            Token::SmallL => {
                let ns = parse_n_nums(&mut tokens, 2)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.line_to(current.0 + ns[0], current.1 + ns[1]);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 2)?;
                    let current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.line_to(current.0 + ns[0], current.1 + ns[1]);
                }
                last_control_point = LastControlPoint::None;
            }
            Token::LargeH => {
                let ns = parse_n_nums(&mut tokens, 1)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.line_to(ns[0], current.1);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 1)?;
                    let current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.line_to(ns[0], current.1);
                }
                last_control_point = LastControlPoint::None;
            }
            Token::SmallH => {
                let ns = parse_n_nums(&mut tokens, 1)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.line_to(current.0 + ns[0], current.1);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 1)?;
                    let current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.line_to(current.0 + ns[0], current.1);
                }
                last_control_point = LastControlPoint::None;
            }
            Token::LargeV => {
                let ns = parse_n_nums(&mut tokens, 1)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.line_to(current.0, ns[0]);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 1)?;
                    let current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.line_to(current.0, ns[0]);
                }
                last_control_point = LastControlPoint::None;
            }
            Token::SmallV => {
                let ns = parse_n_nums(&mut tokens, 1)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.line_to(current.0, current.1 + ns[0]);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 1)?;
                    let current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.line_to(current.0, current.1 + ns[0]);
                }
                last_control_point = LastControlPoint::None;
            }
            Token::LargeQ => {
                let mut ns = parse_n_nums(&mut tokens, 4)?;
                pb.quad(ns[0], ns[1], ns[2], ns[3]);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    ns = parse_n_nums(&mut tokens, 4)?;
                    pb.quad(ns[0], ns[1], ns[2], ns[3]);
                }
                last_control_point = LastControlPoint::Quad(Point::from((ns[0], ns[1])));
            }
            Token::SmallQ => {
                let mut ns = parse_n_nums(&mut tokens, 4)?;
                let mut current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.quad(
                    current.0 + ns[0],
                    current.1 + ns[1],
                    current.0 + ns[2],
                    current.1 + ns[3],
                );
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    ns = parse_n_nums(&mut tokens, 4)?;
                    current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.quad(
                        current.0 + ns[0],
                        current.1 + ns[1],
                        current.0 + ns[2],
                        current.1 + ns[3],
                    );
                }
                last_control_point =
                    LastControlPoint::Quad(Point::from((current.0 + ns[0], current.1 + ns[1])));
            }
            Token::LargeT => {
                let mut ns = parse_n_nums(&mut tokens, 2)?;
                let mut cp = quad_reflection_point(
                    &last_control_point,
                    pb.current_pos().unwrap_or((0.0, 0.0)).into(),
                );
                pb.quad(cp.x(), cp.y(), ns[0], ns[1]);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    cp = Point::from((ns[0], ns[1])) * 2.0 - cp;
                    ns = parse_n_nums(&mut tokens, 2)?;
                    pb.quad(cp.x(), cp.y(), ns[0], ns[1]);
                }
                last_control_point = LastControlPoint::Quad(cp);
            }
            Token::SmallT => {
                let mut ns = parse_n_nums(&mut tokens, 2)?;
                let mut current = pb.current_pos().unwrap_or((0.0, 0.0)).into();
                let mut cp = quad_reflection_point(&last_control_point, current);
                pb.quad(cp.x(), cp.y(), current.x() + ns[0], current.y() + ns[1]);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    cp = (current + Point::from((ns[0], ns[1]))) * 2.0 - cp;
                    current = current + Point::from((ns[0], ns[1]));
                    ns = parse_n_nums(&mut tokens, 2)?;
                    pb.quad(cp.x(), cp.y(), current.x() + ns[0], current.y() + ns[1]);
                }
                last_control_point = LastControlPoint::Quad(cp);
            }
            Token::LargeC => {
                let mut ns = parse_n_nums(&mut tokens, 6)?;
                pb.cubic(ns[0], ns[1], ns[2], ns[3], ns[4], ns[5]);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    ns = parse_n_nums(&mut tokens, 6)?;
                    pb.cubic(ns[0], ns[1], ns[2], ns[3], ns[4], ns[5]);
                }
                last_control_point = LastControlPoint::Cubic(Point::from((ns[2], ns[3])));
            }
            Token::SmallC => {
                let mut ns = parse_n_nums(&mut tokens, 6)?;
                let mut current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.cubic(
                    current.0 + ns[0],
                    current.1 + ns[1],
                    current.0 + ns[2],
                    current.1 + ns[3],
                    current.0 + ns[4],
                    current.1 + ns[5],
                );
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    ns = parse_n_nums(&mut tokens, 6)?;
                    current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.cubic(
                        current.0 + ns[0],
                        current.1 + ns[1],
                        current.0 + ns[2],
                        current.1 + ns[3],
                        current.0 + ns[4],
                        current.1 + ns[5],
                    );
                }
                last_control_point =
                    LastControlPoint::Cubic(Point::from((current.0 + ns[2], current.1 + ns[3])));
            }
            Token::LargeS => {
                let mut ns = parse_n_nums(&mut tokens, 4)?;
                let mut cp = cubic_reflection_point(
                    &last_control_point,
                    pb.current_pos().unwrap_or((0.0, 0.0)).into(),
                );
                pb.cubic(cp.x(), cp.y(), ns[0], ns[1], ns[2], ns[3]);
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    cp = Point::from((ns[2], ns[3])) * 2.0 - Point::from((ns[0], ns[1]));
                    ns = parse_n_nums(&mut tokens, 4)?;
                    pb.cubic(cp.x(), cp.y(), ns[0], ns[1], ns[2], ns[3]);
                }
                last_control_point = LastControlPoint::Quad(Point::from((ns[0], ns[1])));
            }
            Token::SmallS => {
                let mut ns = parse_n_nums(&mut tokens, 4)?;
                let mut current = pb.current_pos().unwrap_or((0.0, 0.0)).into();
                let mut cp = cubic_reflection_point(&last_control_point, current);
                pb.cubic(
                    cp.x(),
                    cp.y(),
                    current.x() + ns[0],
                    current.y() + ns[1],
                    current.x() + ns[2],
                    current.y() + ns[3],
                );
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    cp = current + Point::from((ns[0], ns[1])) * 2.0 - Point::from((ns[2], ns[3]));
                    current = current + Point::from((ns[2], ns[3]));
                    ns = parse_n_nums(&mut tokens, 4)?;
                    pb.cubic(
                        cp.x(),
                        cp.y(),
                        current.x() + ns[0],
                        current.y() + ns[1],
                        current.x() + ns[2],
                        current.y() + ns[3],
                    );
                }
                last_control_point =
                    LastControlPoint::Quad(Point::from((current.x() + ns[0], current.y() + ns[1])));
            }
            Token::LargeA => {
                let mut ns = parse_n_nums(&mut tokens, 7)?;
                pb.ellipse_from_endpoint(
                    ns[0],
                    ns[1],
                    ns[2].to_radians(),
                    ns[3] != 0.0,
                    ns[4] != 0.0,
                    ns[5],
                    ns[6],
                );
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    ns = parse_n_nums(&mut tokens, 7)?;
                    pb.ellipse_from_endpoint(
                        ns[0],
                        ns[1],
                        ns[2].to_radians(),
                        ns[3] != 0.0,
                        ns[4] != 0.0,
                        ns[5],
                        ns[6],
                    );
                }
                last_control_point = LastControlPoint::None;
            }
            Token::SmallA => {
                let mut ns = parse_n_nums(&mut tokens, 7)?;
                let mut current: Point = pb.current_pos().unwrap_or((0.0, 0.0)).into();
                current = current + Point::from((ns[5], ns[6]));
                pb.ellipse_from_endpoint(
                    ns[0],
                    ns[1],
                    ns[2].to_radians(),
                    ns[3] != 0.0,
                    ns[4] != 0.0,
                    current.x(),
                    current.y(),
                );
                while is_num_or_comma(&tokens.peek().unwrap().to_owned()?) {
                    ns = parse_n_nums(&mut tokens, 7)?;
                    current = current + Point::from((ns[5], ns[6]));
                    pb.ellipse_from_endpoint(
                        ns[0],
                        ns[1],
                        ns[2].to_radians(),
                        ns[3] != 0.0,
                        ns[4] != 0.0,
                        current.x(),
                        current.y(),
                    );
                }
                last_control_point = LastControlPoint::None;
            }
            Token::LargeZ | Token::SmallZ => {
                pb.close();
                last_control_point = LastControlPoint::None;
            }
            Token::EOS => {
                break;
            }
            token => {
                return Err(format!("Unexpected token: {:?}", token));
            }
        }
    }
    Ok(pb.end())
}

fn parse_n_nums(tokens: &mut Peekable<Tokenize<Chars>>, n: usize) -> Result<Vec<f64>, String> {
    let mut v = Vec::new();
    for _ in 0..n {
        skip_comma(tokens)?;
        match tokens.next().unwrap()? {
            Token::Num(n) => {
                v.push(n);
            }
            token => {
                return Err(format!("Expects num, but found: {:?}", token));
            }
        };
    }
    Ok(v)
}

fn is_num_or_comma(token: &Token) -> bool {
    match token {
        Token::Num(_) | Token::Comma => true,
        _ => false,
    }
}

fn skip_comma(tokens: &mut Peekable<Tokenize<Chars>>) -> Result<(), String> {
    match tokens.peek().unwrap().to_owned()? {
        Token::Comma => {
            tokens.next().unwrap()?;
        }
        _ => {}
    }
    Ok(())
}

fn quad_reflection_point(lcp: &LastControlPoint, pos: Point) -> Point {
    match lcp {
        LastControlPoint::Quad(p) => pos * 2.0 - *p,
        _ => pos,
    }
}

fn cubic_reflection_point(lcp: &LastControlPoint, pos: Point) -> Point {
    match lcp {
        LastControlPoint::Cubic(p) => pos * 2.0 - *p,
        _ => pos,
    }
}
