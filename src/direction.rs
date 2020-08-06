use crate::geometry::{intersect_segment_and_horizon, intersect_segment_and_vertical};
use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction4 {
    Top,
    Bottom,
    Left,
    Right,
}

pub fn direct4(p1: Point, p2: Point) -> (Direction4, f64) {
    if let Some(p) = intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, p2.1.floor()) {
        return (Direction4::Top, p);
    }
    if let Some(p) = intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, p2.1.floor() + 1.0) {
        return (Direction4::Bottom, p);
    }
    if let Some(p) = intersect_segment_and_vertical(p1.0, p1.1, p2.0, p2.1, p2.0.floor()) {
        return (Direction4::Left, p);
    }
    if let Some(p) = intersect_segment_and_vertical(p1.0, p1.1, p2.0, p2.1, p2.0.floor() + 1.0) {
        return (Direction4::Right, p);
    }
    unreachable!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction8 {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub fn direct8(p1: Point, p2: Point) -> (Direction8, f64) {
    if p1.0 == p2.0 {
        if p1.1 < p2.1 {
            return (Direction8::Bottom, p1.0.floor());
        } else {
            return (Direction8::Top, p1.0.floor());
        }
    }
    if p1.1 == p2.1 {
        if p1.0 < p2.0 {
            return (Direction8::Right, p1.1.floor());
        } else {
            return (Direction8::Left, p1.1.floor());
        }
    }
    if let Some(p) = intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, p2.1.floor()) {
        return (Direction8::Top, p);
    }
    if let Some(p) = intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, p2.1.floor() + 1.0) {
        return (Direction8::Bottom, p);
    }
    if let Some(p) = intersect_segment_and_vertical(p1.0, p1.1, p2.0, p2.1, p2.0.floor()) {
        return (Direction8::Left, p);
    }
    if let Some(p) = intersect_segment_and_vertical(p1.0, p1.1, p2.0, p2.1, p2.0.floor() + 1.0) {
        return (Direction8::Right, p);
    }
    unreachable!()
}
