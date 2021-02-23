pub mod map;
pub use map::*;

use std::collections::LinkedList;

pub struct Hole {
    pub number: usize,
    pub map: Map,
}

pub struct Course {
    holes: LinkedList<Hole>,
}

impl Course {
    pub fn default() -> Self {
        let hole_files = vec!["src/map1.txt", "src/map2.txt"];
        let mut holes: LinkedList<Hole> = LinkedList::new();
        holes.extend(
            hole_files
                .iter()
                .map(|p| Map::load_map(p).unwrap())
                .enumerate()
                .map(|(i, map)| Hole { number: i + 1, map }),
        );
        Self { holes }
    }

    pub fn next(&mut self) -> Option<Hole> {
        self.holes.pop_front()
    }
}
