pub mod map;
pub use map::*;

use std::path::*;
use std::fs::File;
use std::thread::current;
use std::collections::LinkedList;

struct HoleFile {
    path: PathBuf
}

impl HoleFile {
    pub fn new(path: &str) -> Self {
        Self {
            path: Path::new(path).to_owned()
        }
    }
}

pub struct Course {
    current: usize,
    hole_paths: Vec<HoleFile>,
    holes: LinkedList<Map>
}

impl Course {
    pub fn default() -> Self {
        let holes = vec![HoleFile::new("src/map1.txt"), HoleFile::new("src/map2.txt")];
        let mut ll = LinkedList::new();
        ll.extend(holes.iter().map(|h| Map::load_map(h.path.to_str().unwrap()).unwrap()));
        Self {
            current: 0,
            hole_paths: holes,
            holes: ll
        }
    }

    pub fn next(&mut self) -> Option<Map> {
        self.holes.pop_front()
    }
}
