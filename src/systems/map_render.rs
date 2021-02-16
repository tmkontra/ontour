use crate::prelude::*;

pub fn map_render(map: bevy::Res<Map>) {
    let mut draw = DrawBatch::new();
    draw.target(0);
    for y in 0..map.height {
        for x in 0..map.width {
            let t = map.tile_at(x, y);
            draw.set(Point::new(x, y), t.color_pair(), t.glyph());
        }
    }
    draw.submit(10).expect("Batch error");
}
