pub struct Window {
    pub height: u8,
    pub width: u8,
}

impl Window {
    const SCREEN_HEIGHT: u8 = 60;
    const SCREEN_WIDTH: u8 = 80;

    pub fn new() -> Self {
        Self {
            height: Window::SCREEN_HEIGHT,
            width: Window::SCREEN_WIDTH,
        }
    }
}
