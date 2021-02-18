use crate::prelude::*;

pub fn menu(key: Res<Option<VirtualKeyCode>>,
            mut state: ResMut<State<AppState>>) {
    DrawBatch::new()
        .draw_box(
            Rect::with_size(20, 20, 40, 6),
            ColorPair::new(WHITE, BLACK)
        )
        .print(Point::new(30, 22), "Menu! D to play!")
        .submit(15050).expect("Box error");
    match *key {
        Some(VirtualKeyCode::D) => {
            state.set_next(AppState::Playing);
        },
        _ => {}
    };
}
