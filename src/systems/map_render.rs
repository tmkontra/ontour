use crate::prelude::*;

pub fn map_render(map: Res<Map>, camera: Res<Camera>) {
    let mut draw = DrawBatch::new();
    let OOB: ColorPair = ColorPair::new(WHITE, GRAY0);
    draw.target(0);
    for y in camera.y_iter() {
        for x in camera.x_iter() {
            let p = Point::new(x, y);
            if map.in_bounds(p) {
                let t = map.tile_at(x as u8, y as u8);
                let pix = camera.render_coordinate(Point::new(x, y));
                draw.set(pix, t.color_pair(), t.glyph());
            } else {
                draw.set(camera.render_coordinate(Point::new(x, y)), OOB, to_cp437('░'));
            }
        }
    }
    draw.submit(2020).expect("Batch error");
}
