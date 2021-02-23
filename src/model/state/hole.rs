#[derive(Copy, Clone, Debug)]
pub enum HoleState {
    TeeOff,
    Stroke(u32),
    Holed,
}

impl HoleState {
    pub fn new() -> Self {
        HoleState::TeeOff
    }

    pub fn start_hole() -> Self {
        HoleState::Stroke(0)
    }

    pub fn increment(&mut self) {
        match self {
            HoleState::Stroke(strokes) => *strokes += 1,
            _other => (),
        }
    }
}
