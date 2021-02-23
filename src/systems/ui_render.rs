use crate::prelude::*;

pub fn render_ui(
    turn_stage: Res<TurnStage>,
    hole: Res<Hole>,
    camera: Res<Camera>,
    _key: Res<Option<VirtualKeyCode>>,
    balls: Query<&Ball>,
    window: Res<Window>,
    hole_state: Res<HoleState>,
) {
    let mut ctx = DrawBatch::new();
    let map = &hole.map;

    let mut render_swing = |ball: &Ball, swing: Swing, direction: &f32| match swing {
        Swing::Start => {
            ctx.print(
                Point::new(2, window.height - 3),
                "[Start] Aim, Press Space to Start Swing!",
            );
            let coord = crosshair_coord(ball.tile_position(), direction);
            if map.in_bounds(&coord) {
                let bg = map.bg(&coord);
                ctx.set(
                    camera.render_coordinate(&coord),
                    ColorPair::new(WHITE, bg),
                    9,
                );
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
            if map.in_bounds(&coord) {
                let bg = map.bg(&coord);
                ctx.set(
                    camera.render_coordinate(&coord),
                    ColorPair::new(WHITE, bg),
                    9,
                );
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
            if map.in_bounds(&coord) {
                let bg = map.bg(&coord);
                ctx.set(
                    camera.render_coordinate(&coord),
                    ColorPair::new(WHITE, bg),
                    9,
                );
            } else {
                let arr = map.intersection(ball.tile_position(), coord);
                ctx.set(arr, ColorPair::new(RED, BLACK), 30);
            }
        }
    };
    let instr = match *hole_state {
        HoleState::TeeOff => "Start The Hole!".to_string(),
        HoleState::Stroke(strokes) => format!("Strokes: {}", strokes),
        HoleState::Holed => "Hole finished!".to_string(),
    };
    match *turn_stage {
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
                if map.in_bounds(&coord) {
                    let bg = map.bg(&coord);
                    ctx.set(
                        camera.render_coordinate(&coord),
                        ColorPair::new(WHITE, bg),
                        9,
                    );
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
        TurnStage::Traveling(_travel) => {
            for _ball in balls.iter() {
                ctx.print(Point::new(2, window.height - 3), "Traveling!");
            }
        }
        TurnStage::Finished => {
            ctx.print(Point::new(2, window.height - 3), "Finishing Turn");
        }
    }

    let (ui_h0, ui_h2) = (camera.height() - 1, window.height as i32 - 1);
    let ui_w = window.width - 1;
    DrawBatch::new()
        .draw_box(
            Rect::with_exact(0, ui_h0, ui_w as i32, ui_h2),
            ColorPair::new(WHITE, BLACK),
        )
        .draw_box(
            Rect::with_exact(0, 0, camera.width() - 1, camera.height() - 1),
            ColorPair::new(WHITE, BLACK),
        )
        .draw_box(
            Rect::with_exact(camera.width(), 0, ui_w as i32, camera.height() - 1),
            ColorPair::new(WHITE, BLACK),
        )
        .print(Point::new(camera.width() + 1, 2), instr)
        .submit(1010)
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
