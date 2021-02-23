use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ball {
    x: f32,
    y: f32,
}

impl Ball {
    pub fn new(position: &Point) -> Ball {
        Self {
            x: position.x as f32,
            y: position.y as f32,
        }
    }

    pub fn tile_position(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }

    pub fn mv(&mut self, deg: f32, dist: f32) {
        let rads = (deg + 90.).to_radians();
        let dx = rads.cos() * dist;
        let dy = -1. * rads.sin() * dist;
        self.x += dx;
        self.y += dy;
    }
}
