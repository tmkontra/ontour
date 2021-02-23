use crate::model::util::Rectangle;
use crate::prelude::*;

pub struct Camera {
    rect: Rectangle,
    map_x: i32,
    map_y: i32,
    display_width: i32,
    display_height: i32,
    pub map_coords: Vec<Point>,
}

impl Camera {
    pub fn new(
        position: Point,
        map_x: i32,
        map_y: i32,
        display_width: i32,
        display_height: i32,
    ) -> Self {
        let y = if map_y - position.y < display_height / 2 {
            let dy = (display_height / 2) - (map_y - position.y);
            position.y - dy
        } else if position.y < display_height / 2 {
            display_height / 2
        } else {
            position.y
        };
        let x = if map_x - position.x < display_width / 2 {
            let dx = (display_width / 2) - (map_x - position.x);
            position.x - dx
        } else if position.x < display_width / 2 {
            display_width / 2
        } else {
            position.x
        };
        let rect = Rectangle::new(Point::new(x, y), display_width, display_height);
        let map_coords: Vec<Point> = rect.coordinates();
        Self {
            display_width,
            display_height,
            rect,
            map_x,
            map_y,
            map_coords,
        }
    }

    pub fn width(&self) -> i32 {
        self.rect.width()
    }

    pub fn height(&self) -> i32 {
        self.rect.height()
    }

    pub fn update(&mut self, position: Point) {
        let y = if self.map_y - position.y < self.display_height / 2 {
            let dy = (self.display_height / 2) - (self.map_y - position.y);
            position.y - dy
        } else if position.y < self.display_height / 2 {
            self.display_height / 2
        } else {
            position.y
        };
        let x = if self.map_x - position.x < self.display_width / 2 {
            let dx = (self.display_width / 2) - (self.map_x - position.x);
            position.x - dx
        } else if position.x < self.display_width / 2 {
            self.display_width / 2
        } else {
            position.x
        };
        let rect = Rectangle::new(Point::new(x, y), self.display_width, self.display_height);
        let map_coords: Vec<Point> = rect.coordinates();
        self.rect = rect;
        self.map_coords = map_coords;
    }

    pub fn render_coordinate(&self, position: &Point) -> Point {
        self.rect.relative_point(position)
    }
}
