use crate::prelude::*;

pub fn render_ui(turnStage: Res<TurnStage>,
                 map: Res<Map>,
                 key: Res<Option<VirtualKeyCode>>,
                 balls: Query<&Ball>
) {
    let mut ctx = DrawBatch::new();

    let mut render_swing = |ball: &Ball, swing: Swing| {
        match swing {
            Swing::Start(deg) => {
                ctx.print(Point::new(2, SCREEN_HEIGHT - 3), "[Start] Aim, Press Space to Start Swing!");
                let coord  = crosshair_coord(ball.tile_position(), swing.direction());
                if map.in_bounds(coord) {
                    let bg = map.bg(coord);
                    ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
                } else {
                    let arr = map.intersection(ball.tile_position(), coord);
                    ctx.set(arr, ColorPair::new(RED, BLACK), 30);
                }
            },
            Swing::Power(deg, pow) => {
                ctx.print(Point::new(2, SCREEN_HEIGHT - 3), "[Power] Aim, Press Space to Start Swing!");
                let coord  = crosshair_coord(ball.tile_position(), swing.direction());
                if map.in_bounds(coord) {
                    let bg = map.bg(coord);
                    ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
                } else {
                    let arr = map.intersection(ball.tile_position(), coord);
                    ctx.set(arr, ColorPair::new(RED, BLACK), 30);
                }
                ctx.bar_horizontal(Point::new(2, SCREEN_HEIGHT - 10),
                                   51,
                                   pow as i32,
                                   100,
                                   ColorPair::new(RED, BLACK));
            },
            Swing::Accuracy(_, _, _) => {
                ctx.print(Point::new(2, SCREEN_HEIGHT - 3), "[Acc] Aim, Press Space to Start Swing!");
                let coord  = crosshair_coord(ball.tile_position(), swing.direction());
                if map.in_bounds(coord) {
                    let bg = map.bg(coord);
                    ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
                } else {
                    let arr = map.intersection(ball.tile_position(), coord);
                    ctx.set(arr, ColorPair::new(RED, BLACK), 30);
                }
            },
        }
    };
    match *turnStage {
        TurnStage::Aiming(Aim { degrees }) => {
            for ball in balls.iter() {
                let coord  = crosshair_coord(ball.tile_position(), &degrees);
                if map.in_bounds(coord) {
                    let bg = map.bg(coord);
                    ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
                } else {
                    let arr = map.intersection(ball.tile_position(), coord);
                    ctx.set(arr, ColorPair::new(RED, BLACK), 30);
                }
            };
            ctx.print(Point::new(2, SCREEN_HEIGHT - 3), "Aiming");
        },
        TurnStage::Swinging(swing) => {
            for ball in balls.iter() {
                render_swing(ball, swing.clone());
            }
        },
        TurnStage::Traveling(_) => {
            for ball in balls.iter() {
                ctx.print(
                    Point::new(2, SCREEN_HEIGHT - 3),
                    format!("Ball is Traveling {}", ball.velocity)
                );
            }
        },
        TurnStage::Finished => {
            ctx.print(Point::new(2, SCREEN_HEIGHT - 3), "Finishing Turn");
        },
    }

    DrawBatch::new()
        .draw_box(Rect::with_size(0, 43, 79, 6), ColorPair::new(WHITE, BLACK))
        .submit(15050).expect("Box error");
    ctx.submit(20220).expect("UI Error!");
}


fn crosshair_coord(origin: Point, degrees: &f32) -> Point {
    let radius = 20.;
    let rads = (*degrees + 90.).to_radians();
    let dx = rads.cos() * radius;
    let dy = -1. * rads.sin() * radius;
    let dir = Point::new(dx.round() as i32, dy.round() as i32);
    origin + dir
}
