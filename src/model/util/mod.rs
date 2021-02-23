use crate::prelude::*;

pub struct Rectangle {
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
}

impl Rectangle {
    pub fn new(origin: Point, width: i32, height: i32) -> Self {
        let x0 = origin.x - width / 2;
        let x1 = origin.x + width / 2;
        let y0 = origin.y - height / 2;
        let y1 = origin.y + height / 2;
        Self { x0, x1, y0, y1 }
    }

    pub fn width(&self) -> i32 {
        self.x1 - self.x0
    }

    pub fn height(&self) -> i32 {
        self.y1 - self.y0
    }

    pub fn coordinates(&self) -> Vec<Point> {
        (self.y0..self.y1 - 2)
            .cartesian_product(self.x0..self.x1 - 2)
            .map(|(y, x)| Point::new(x, y))
            .collect()
    }

    pub fn relative_point(&self, position: &Point) -> Point {
        Point::new(position.x - self.x0 + 1, position.y - self.y0 + 1)
    }
}
