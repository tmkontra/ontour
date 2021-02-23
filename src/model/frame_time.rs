pub struct FrameTime {
    pub t_ms: f32,
}

impl FrameTime {
    pub fn new() -> Self {
        FrameTime { t_ms: 0. }
    }

    pub fn of(ms: f32) -> Self {
        FrameTime { t_ms: ms }
    }

    pub fn seconds(&self) -> f32 {
        self.t_ms / 1000.
    }
}
