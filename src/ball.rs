use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ball {
    x: f32,
    y: f32,
    pub velocity: f32,
    pub direction: f32,
    frame_time: f32
}

impl Ball {
    pub fn new(position: &Point) -> Ball {
        Self {
            x: position.x as f32,
            y: position.y as f32,
            velocity: 0.,
            direction: 0.,
            frame_time: 0.
        }
    }

    pub fn tile_position(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }

    fn decel(&mut self) {
        let dec = (self.velocity * 0.9);
        let fric = if dec < 2. {
            (dec * 0.7) - 0.2
        } else {
            dec
        };
        let r = if fric < 1. {
            0.
        } else {
            fric
        };
        self.velocity = r;
    }

    fn mv(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    fn motion(&mut self) {
        println!("dir: {:?}", self.direction);
        println!("dird: {:?}", self.direction + 90.);
        let rads = (self.direction + 90.).to_radians();
        let dx = rads.cos();
        let dy = -1. * rads.sin().ceil();
        println!("{:?}, {:?}", dx, dy);
        self.mv(dx, dy);
    }

    pub fn stopped(&self) -> bool {
        self.velocity <= 0.
    }

    pub fn tick(&mut self, ctx: &mut BTerm) {
        if self.velocity > 0. {
            self.frame_time += ctx.frame_time_ms;
            if self.frame_time > FRAME_DURATION / self.velocity {
                self.frame_time = 0.;
                self.motion();
                self.decel();
            }
        }
    }
}
