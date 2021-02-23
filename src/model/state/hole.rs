#[derive(Copy, Clone, Debug)]
pub enum HoleState {
    Start,
    Stroke(u32),
    Holed,
}

impl HoleState {
    pub fn new() -> Self {
        HoleState::Start
    }

    pub fn startHole() -> Self {
        HoleState::Stroke(0)
    }

    pub fn finish() -> Self {
        HoleState::Holed
    }

    pub fn increment(&mut self) {
        match self {
            HoleState::Stroke(strokes) => *strokes += 1,
            other => (),
        }
    }
}
