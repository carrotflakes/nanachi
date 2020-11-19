//! Segments

use crate::path::PathItem;
use crate::point::Point;

pub struct Segments<I: Iterator<Item = PathItem>> {
    loop_start: Option<Point>,
    last: Point,
    path_items: I,
}

impl<I: Iterator<Item = PathItem>> Segments<I> {
    pub fn new(it: I) -> Self {
        Segments {
            loop_start: None,
            last: Point(0.0, 0.0),
            path_items: it,
        }
    }
}

impl<I: Iterator<Item = PathItem>> Iterator for Segments<I> {
    type Item = (Point, Point);

    fn next(&mut self) -> Option<Self::Item> {
        match self.path_items.next() {
            Some(PathItem::Line(l)) => {
                if self.loop_start == None {
                    self.loop_start = Some(l.0);
                }
                self.last = l.1;
                Some((l.0, l.1))
            }
            Some(PathItem::CloseAndJump) => {
                self.loop_start = None;
                self.next()
            }
            Some(PathItem::Jump) => {
                let loop_start = self.loop_start.unwrap();
                self.loop_start = None;
                if loop_start == self.last {
                    self.next()
                } else {
                    Some((self.last, loop_start))
                }
            }
            None => {
                if let Some(loop_start) = self.loop_start {
                    self.loop_start = None;
                    Some((self.last, loop_start))
                } else {
                    None
                }
            }
            _ => panic!(),
        }
    }
}
