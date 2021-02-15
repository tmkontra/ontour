use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map) {
    let mut draw = DrawBatch::new();
    draw.target(0);
    for y in 0..map.height {
        for x in 0..map.width {
            let t = map.tile_at(x, y);
            draw.set(Point::new(x, y), t.color_pair(), t.glyph());
        }
    }
    draw.submit(0).expect("Batch error");
}
