use crate::prelude::*;

pub fn ball_render(balls: Query<&Ball>, hole: Res<Hole>, camera: Res<Camera>) {
    let mut draw = DrawBatch::new();
    let map = &hole.map;
    draw.target(0);
    balls.iter().for_each(|ball: &Ball| {
        let pos = ball.tile_position();
        let bg = map.bg(&ball.tile_position());
        let pix = camera.render_coordinate(&pos);
        draw.set(pix, ColorPair::new(WHITE, bg), 7);
    });
    draw.submit(10100).expect("Batch error");
}
