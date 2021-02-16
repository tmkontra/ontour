use crate::prelude::*;


pub fn ball_render(balls: Query<&Ball>, map: Res<Map>) {
    let mut draw = DrawBatch::new();
    draw.target(0);
    balls.iter()
        .for_each(|ball: &Ball| {
            let bg = map.bg(ball.tile_position());
            println!("Ball at: {:?}", ball.tile_position());
            draw.set(ball.tile_position(), ColorPair::new(WHITE, bg), 7);
        });
    draw.submit(10100).expect("Batch error");
}
