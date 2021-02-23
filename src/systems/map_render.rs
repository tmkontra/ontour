use crate::prelude::*;

pub fn map_render(hole: Res<Hole>, camera: Res<Camera>) {
    let mut draw = DrawBatch::new();
    let OOB: ColorPair = ColorPair::new(WHITE, GRAY0);
    let map = &hole.map;
    draw.target(0);
    for point in &camera.map_coords {
        if map.in_bounds(point) {
            let t = map.tile_at(point);
            let pix = camera.render_coordinate(point);
            draw.set(pix, t.color_pair(), t.glyph());
        }
    }
    draw.submit(2020).expect("Batch error");
}
