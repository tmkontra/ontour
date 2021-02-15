use crate::prelude::*;


pub fn ball_render_b() {
    let mut draw = DrawBatch::new();
    draw.target(0);
    // <&Ball>::query() .iter(ecs)
    //     .for_each(|ball: &Ball| {
    //         draw.set(ball.tile_position(), ColorPair::new(WHITE, BLACK), 7);
    //     });
    draw.submit(1).expect("Batch error");
}
