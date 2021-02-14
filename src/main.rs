use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use bracket_lib::prelude::*;
use std::ops::{Deref, Add};

struct State {
    pub message: String,
    SCREEN_HEIGHT: u8,
    SCREEN_WIDTH: u8,
    pub map: Map,
    pub ball: Ball,
    mode: Mode,
}

struct Aim {
    degrees: f32
}

impl Aim {
    const RATE: f32 = 3.;

    pub fn new() -> Self {
        Self { degrees: 0. }
    }

    fn aim(&mut self, key: Option<VirtualKeyCode>) {
        match key {
            Some(VirtualKeyCode::Left) => {
                self.degrees += Aim::RATE;
            },
            Some(VirtualKeyCode::Right) => {
                self.degrees -= Aim::RATE;
            },
            _ => {}
        }
    }
}

struct Swing {
    degrees: f32,
    power: Option<f32>
}

struct Travel {

}

struct Finished {

}

enum Mode {
    Aiming(Aim),
    Swinging(Swing),
    Traveling(Travel),
    Finished
}

impl Mode {
    pub fn default() -> Mode {
        Mode::Aiming(Aim::new())
    }

    fn start_swing(degrees: f32) -> Mode {
        Mode::Swinging(Swing { degrees, power: None })
    }

    pub fn next(&self) -> Mode {
        match self {
            Mode::Aiming(Aim { degrees }) => Mode::start_swing(degrees.clone()),
            Mode::Swinging(_) => Mode::Traveling(Travel{}),
            _ => Mode::Finished
        }
    }
}

struct Ball {
    x: f32,
    y: f32,
    velocity: f32,
    direction: f32,
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

    fn position(&self) -> Point {
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

    fn stopped(&self) -> bool {
        self.velocity <= 0.
    }

    fn render(&mut self, bg: (u8, u8, u8), ctx: &mut BTerm) {
        let pos = self.position();
        ctx.set(pos.x, pos.y, WHITE, bg, 7);
        if self.velocity > 0. {
            self.frame_time += ctx.frame_time_ms;
            if self.frame_time > State::FRAME_DURATION / self.velocity {
                self.frame_time = 0.;
                self.motion();
                self.decel();
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Tee,
    TeeBox,
    Fairway,
    Green,
    Flag,
    Rough
}

impl Tile {
    pub fn from_char(c: &char) -> Tile {
        match c {
            'T' => Tile::Tee,
            'D' => Tile::TeeBox,
            '=' => Tile::Fairway,
            '@' => Tile::Green,
            'F' => Tile::Flag,
            _ => Tile::Rough
        }
    }

    pub fn render(self) -> u16 {
        let c = match self {
            Tile::Tee => 'T',
            Tile::TeeBox => '█',
            Tile::Fairway => '█',
            Tile::Green => '█',
            Tile::Flag => 'F',
            Tile::Rough => '░',
        };
        to_cp437(c)
    }

    pub fn color(self) -> (u8, u8, u8) {
        match self {
            Tile::Tee => WHITE,
            Tile::TeeBox => DARKGREEN,
            Tile::Fairway => LAWN_GREEN,
            Tile::Green => LIGHTGREEN,
            Tile::Flag => RED,
            Tile::Rough => DARKGREEN,
        }
    }

    pub fn bg(self) -> (u8, u8, u8) {
        match self {
            Tile::Tee => DARKGREEN,
            Tile::Flag => LIGHTGREEN,
            _ => self.color(),
        }
    }
}

struct Map {
    width: u8,
    height: u8,
    points: Vec<Tile>,
    pub tee: Point,
    pub flag: Point,
}

impl Map {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            points: Vec::<Tile>::new(),
            tee: Point::zero(),
            flag: Point::zero(),
        }
    }

    pub fn load_map(width: u8, height: u8, filename: &str) -> Option<Self> {
        let f: File = File::open(filename).unwrap();
        let l: Lines<BufReader<File>> = BufReader::new(f).lines();
        let mut buf = vec![Tile::Rough; height as usize * width as usize];
        let mut tee = None;
        let mut flag = None;
        for (y, line) in l.take(height as usize).enumerate() {
            let ln = line.unwrap();
            for (x, c) in ln.chars().take(width as usize).enumerate() {
                let tile = Tile::from_char(&c);
                match tile {
                    Tile::Tee => {
                        if tee.is_some() {
                            panic!("Too many tees!")
                        } else {
                            tee = Some(Point::new(x, y));
                        }
                    },
                    Tile::Flag => {
                        if flag.is_some() {
                            panic!("Too many flags!")
                        } else {
                            flag = Some(Point::new(x, y));
                        }
                    },
                    _ => {}
                }
                let n = (y * width as usize) + x;
                buf[n] = tile;
            }
        }
        Some(Self {
            width,
            height,
            points: buf,
            tee: tee?,
            flag: flag?
        })
    }

    pub fn tile_at(&self, x: u8, y: u8) -> Tile {
        let n = ((y as u16 * self.width as u16) + x as u16) as usize;
        self.points[n]
    }

    pub fn bg(&self, position: Point) -> (u8, u8, u8) {
        self.tile_at(position.x as u8, position.y as u8).bg()
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..self.height {
            for x in 0..self.width {
                let t = self.tile_at(x, y);
                ctx.set(x, y, t.color(), t.bg(), t.render())
            }
        }
    }
}

impl State {
    const FRAME_DURATION: f32 = 300.;
    const SCREEN_HEIGHT: u8 = 50;
    const SCREEN_WIDTH: u8 = 80;

    fn new() -> Self {
        let map = Map::load_map(Self::SCREEN_WIDTH, Self::SCREEN_HEIGHT, "src/map1.txt").unwrap();
        let ball = Ball::new(&map.tee);
        Self {
            message: "ON TOUR!".to_string(),
            SCREEN_WIDTH: State::SCREEN_WIDTH,
            SCREEN_HEIGHT: State::SCREEN_HEIGHT,
            map,
            ball,
            mode: Mode::default()
        }
    }

    fn draw_ui(&self, ctx: &mut BTerm) {
        ctx.draw_box(0, 43, 79, 6, RGB::named(WHITE), RGB::named(BLACK));
    }

    fn crosshair(origin: Point, degrees: f32) -> Point {
        let radius = 20.;
        let rads = (degrees + 90.).to_radians();
        let dx = rads.cos() * radius;
        let dy = -1. * rads.sin() * radius;
        let dir = Point::new(dx.round() as i32, dy.round() as i32);
        origin + dir
    }

    fn transition(&mut self, key: Option<VirtualKeyCode>) {
        let next = self.mode.next();
        match (&mut self.mode, &next, key) {
            (Mode::Swinging(swing), Mode::Traveling(_), Some(VirtualKeyCode::Space)) => {
                match swing.power {
                    None => {
                        swing.power = Some(20.);
                    },
                    Some(power) => {
                        self.ball.direction = swing.degrees;
                        self.ball.velocity = power;
                        self.mode = next
                    },
                }
            },
            (Mode::Traveling(_), _, _) => {
                if self.ball.stopped() {
                    self.mode = next
                }
            },
            (_, _, Some(VirtualKeyCode::Space)) => self.mode = next,
            _ => {}
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);

        self.ball.render(self.map.bg(self.ball.position()), ctx);
        self.draw_ui(ctx);
        match &mut self.mode {
            Mode::Aiming(aim) => {
                ctx.print(2, self.SCREEN_HEIGHT - 3, "Aiming");
                let coord  = State::crosshair(self.ball.position(), aim.degrees);
                let bg = self.map.bg(coord);
                ctx.set(coord.x, coord.y, WHITE, bg, 9);
                aim.aim(ctx.key)
            },
            Mode::Swinging(swing) => {
                ctx.print(2, self.SCREEN_HEIGHT - 3, "Swinging!");
                let coord  = State::crosshair(self.ball.position(), swing.degrees);
                let bg = self.map.bg(coord);
                ctx.set(coord.x, coord.y, WHITE, bg, 9);
            },
            Mode::Traveling(_) => {
                ctx.print(2, self.SCREEN_HEIGHT - 3, format!("Ball is Traveling {}", self.ball.velocity))
            },
            Mode::Finished => {
                ctx.print(2, self.SCREEN_HEIGHT - 3, "Finishing Turn")
            },
        }
        self.transition(ctx.key);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("ON TOUR")
        .with_fps_cap(30.0)
        .build()?;

    let s = State::new();

    main_loop(context, s)
}
