use crate::prelude::*;

pub fn render_ui(
    turnStage: Res<TurnStage>,
    map: Res<Map>,
    key: Res<Option<VirtualKeyCode>>,
    balls: Query<&Ball>,
    window: Res<Window>,
) {
    let mut ctx = DrawBatch::new();

    let mut render_swing = |ball: &Ball, swing: Swing, direction: &f32| match swing {
        Swing::Start => {
            ctx.print(
                Point::new(2, window.height - 3),
                "[Start] Aim, Press Space to Start Swing!",
            );
            let coord = crosshair_coord(ball.tile_position(), direction);
            if map.in_bounds(coord) {
                let bg = map.bg(coord);
                ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
            } else {
                let arr = map.intersection(ball.tile_position(), coord);
                ctx.set(arr, ColorPair::new(RED, BLACK), 30);
            }
        }
        Swing::Power(pow) => {
            ctx.print(
                Point::new(2, window.height - 3),
                "[Power] Aim, Press Space to Start Swing!",
            );
            let coord = crosshair_coord(ball.tile_position(), direction);
            if map.in_bounds(coord) {
                let bg = map.bg(coord);
                ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
            } else {
                let arr = map.intersection(ball.tile_position(), coord);
                ctx.set(arr, ColorPair::new(RED, BLACK), 30);
            }
            ctx.bar_horizontal(
                Point::new(2, window.height - 10),
                51,
                pow as i32,
                100,
                ColorPair::new(RED, BLACK),
            );
        }
        Swing::Accuracy(_, _) => {
            ctx.print(
                Point::new(2, window.height - 3),
                "[Acc] Aim, Press Space to Start Swing!",
            );
            let coord = crosshair_coord(ball.tile_position(), direction);
            if map.in_bounds(coord) {
                let bg = map.bg(coord);
                ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
            } else {
                let arr = map.intersection(ball.tile_position(), coord);
                ctx.set(arr, ColorPair::new(RED, BLACK), 30);
            }
        }
    };
    match *turnStage {
        TurnStage::ClubSelection(clubs, current) => {
            let club = clubs.at(&current);
            println!("Current club: {:?} = {:?}", &current, club.name);
            ctx.print(
                Point::new(2, window.height - 3),
                format!("Club selected: {}", club.name),
            );
        }
        TurnStage::Aiming(Aim { degrees }, _) => {
            for ball in balls.iter() {
                let coord = crosshair_coord(ball.tile_position(), &degrees);
                if map.in_bounds(coord) {
                    let bg = map.bg(coord);
                    ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
                } else {
                    let arr = map.intersection(ball.tile_position(), coord);
                    ctx.set(arr, ColorPair::new(RED, BLACK), 30);
                }
            }
            ctx.print(Point::new(2, window.height - 3), "Aiming");
        }
        TurnStage::Swinging(swing, aim, _) => {
            for ball in balls.iter() {
                render_swing(ball, swing.clone(), &aim.degrees);
            }
        }
        TurnStage::Traveling(travel) => {
            for ball in balls.iter() {
                ctx.print(
                    Point::new(2, window.height - 3),
                    format!("Ball is Traveling vi {}", travel.initial_velocity),
                );
            }
        }
        TurnStage::Finished => {
            ctx.print(Point::new(2, window.height - 3), "Finishing Turn");
        }
    }

    let (uiH0, uiH) = (map.height, window.height - 1 - map.height);
    let uiW = window.width - 1;
    DrawBatch::new()
        .draw_box(
            Rect::with_size(0, uiH0, uiW, uiH),
            ColorPair::new(WHITE, BLACK),
        )
        .submit(15050)
        .expect("Box error");
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
