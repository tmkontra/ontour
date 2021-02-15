use crate::prelude::*;
use std::fs::File;
use std::io::*;

#[derive(Debug, Clone)]
pub struct Map {
    pub width: u8,
    pub height: u8,
    points: Vec<MapTile>,
    pub tee: Point,
    pub flag: Point,
}

impl Map {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            points: Vec::<MapTile>::new(),
            tee: Point::zero(),
            flag: Point::zero(),
        }
    }

    pub fn load_map(width: u8, height: u8, filename: &str) -> Option<Self> {
        let f: File = File::open(filename).unwrap();
        let l: BufReader<File> = BufReader::new(f);
        let mut buf = vec![MapTile::Rough; height as usize * width as usize];
        let mut tee = None;
        let mut flag = None;
        for (y, line) in l.lines().take(height as usize).enumerate() {
            let ln = line.unwrap();
            for (x, c) in ln.chars().take(width as usize).enumerate() {
                let tile = MapTile::from_char(&c);
                match tile {
                    MapTile::Tee => {
                        if tee.is_some() {
                            panic!("Too many tees!")
                        } else {
                            tee = Some(Point::new(x, y));
                        }
                    },
                    MapTile::Flag => {
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

    pub fn tile_at(&self, x: u8, y: u8) -> MapTile {
        let n = ((y as u16 * self.width as u16) + x as u16) as usize;
        self.points[n]
    }

    pub fn bg(&self, position: Point) -> (u8, u8, u8) {
        self.tile_at(position.x as u8, position.y as u8).bg()
    }
}
