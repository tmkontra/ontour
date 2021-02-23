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
    pub fn load_map(filename: &str) -> Option<Self> {
        let p = std::env::current_dir();
        println!("Path: {:?}", p);
        let f: File = File::open(filename).unwrap();
        let l: BufReader<File> = BufReader::new(f);
        let mut tee = None;
        let mut flag = None;
        let lines: Vec<String> = l.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
        let width: u8 = lines.iter().take(1).next().unwrap().chars().count() as u8;
        let height: u8 = lines.len() as u8;
        let mut buf = vec![MapTile::Rough; height as usize * width as usize];
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().take(width as usize).enumerate() {
                let tile = MapTile::from_char(&c);
                match tile {
                    MapTile::Tee => {
                        if tee.is_some() {
                            panic!("Too many tees!")
                        } else {
                            tee = Some(Point::new(x, y));
                        }
                    }
                    MapTile::Flag => {
                        if flag.is_some() {
                            panic!("Too many flags!")
                        } else {
                            flag = Some(Point::new(x, y));
                        }
                    }
                    _ => {}
                }
                let n = (y * width as usize) + x;
                buf[n] = tile;
            }
        }
        let mut map = Self {
            width,
            height,
            points: buf.clone(),
            tee: tee?,
            flag: flag?,
        };
        let mut fairway_adjacency = vec![0 as usize; height as usize * width as usize];
        for (i, point) in map.points.iter().enumerate() {
            if let MapTile::DeepRough = point {
                let y = i as i32 / map.width as i32;
                let x = i as i32 % map.width as i32;
                let deep_rad = 8;
                let min_x = (x - deep_rad).max(0);
                let min_y = (y - deep_rad).max(0);
                let max_x = (x + deep_rad).min(width as i32 - 1);
                let max_y = (y + deep_rad).min(height as i32 - 1);
                for ix in min_x..max_x {
                    for iy in min_y..max_y {
                        if let MapTile::Fairway = &map.tile_at_xy(ix as u8, iy as u8) {
                            fairway_adjacency[i] += 1;
                        }
                    }
                }
            }
        }
        for (i, near_fairway) in fairway_adjacency.iter().enumerate() {
            if *near_fairway > 3 {
                buf[i] = MapTile::Rough;
            }
        }
        map.points = buf;
        Some(map)
    }

    pub fn intersection(&self, p1: Point, p2: Point) -> Point {
        let y = if p2.y > self.height as i32 {
            // bottom edge
            Some(self.height as i32)
        } else if p2.y < 0 {
            // top edge
            Some(0)
        } else {
            None
        };
        let x = if p2.x > self.width as i32 {
            Some(self.width as i32)
        } else if p2.x < 0 {
            Some(0)
        } else {
            None
        };
        match (x, y) {
            (Some(x), Some(y)) => Point::new(x, y),
            (Some(x), None) => {
                let dy = p2.y - p1.y;
                let dx = p2.x - p1.x;
                let m = dy as f32 / dx as f32;
                let xdiff = x - p1.x;
                let y = (p1.y as f32 + (xdiff as f32 * m)) as i32;
                println!("Known x {:?}, calc y {:?}", x, y);
                Point::new(x, y)
            }
            (None, Some(y)) => {
                if p2.x == p1.x {
                    Point::new(p1.x, y)
                } else {
                    let dy = p2.y - p1.y;
                    let dx = p2.x - p1.x;
                    let m = dy as f32 / dx as f32;
                    let ydiff = y - p1.y;
                    let x = (p1.x as f32 + (ydiff as f32 / m)) as i32;
                    println!("Known y {:?}, calc x {:?}", y, x);
                    Point::new(x, y)
                }
            }
            _ => Point::new(0, 0),
        }
    }

    pub fn in_bounds(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32
    }

    pub fn tile_at(&self, point: &Point) -> MapTile {
        let n = ((point.y * self.width as i32) + point.x as i32) as usize;
        self.points[n]
    }

    pub fn tile_at_xy(&self, x: u8, y: u8) -> MapTile {
        let n = ((y as u16 * self.width as u16) + x as u16) as usize;
        self.points[n]
    }

    pub fn bg(&self, position: &Point) -> (u8, u8, u8) {
        self.tile_at(position).bg()
    }
}
