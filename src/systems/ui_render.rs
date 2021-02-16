use crate::prelude::*;

pub fn render_ui(mut mode: ResMut<Mode>,
                 map: Res<Map>, key: Res<Option<VirtualKeyCode>>,
                 balls: Query<&Ball>
) {
    let mut ctx = DrawBatch::new();

    let mut render_crosshair = |ctx: &mut DrawBatch, ball: Ball, angle: &f32| {

    };
    let mut render_swing = |ball: Ball, swing: Swing| {
        match swing {
            Swing::Start(deg) => {
                ctx.print(Point::new(2, SCREEN_HEIGHT - 3), "[Start] Aim, Press Space to Start Swing!");
                let coord  = crosshair(ball.tile_position(), swing.direction());
                let bg = map.bg(coord);
                ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
            },
            Swing::Power(deg, pow) => {
                ctx.print(Point::new(2, SCREEN_HEIGHT - 3), "[Power] Aim, Press Space to Start Swing!");
                let coord  = crosshair(ball.tile_position(), swing.direction());
                let bg = map.bg(coord);
                ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
                ctx.bar_horizontal(Point::new(2, SCREEN_HEIGHT - 10),
                                   51,
                                   pow as i32,
                                   100,
                                   ColorPair::new(RED, BLACK));
            },
            Swing::Accuracy(_, _, _) => {
                ctx.print(Point::new(2, SCREEN_HEIGHT - 3), "[Acc] Aim, Press Space to Start Swing!");
                let coord  = crosshair(ball.tile_position(), swing.direction());
                let bg = map.bg(coord);
                ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
            },
        }
    };

    match *mode {
        Mode::Aiming(aim) => {
            ctx.print(Point::new(2, SCREEN_HEIGHT - 3), "Aiming");
            let new_aim = aim.aim(*key);
            for &ball in balls.iter() {
                let coord  = crosshair(ball.tile_position(), &new_aim);
                let bg = map.bg(coord);
                ctx.set(Point::new(coord.x, coord.y), ColorPair::new(WHITE, bg), 9);
            }
            *mode = Mode::Aiming(Aim { degrees: new_aim });
        },
        Mode::Swinging(swing) => {
            let new_swing = handle_swing(swing.clone());
            for &ball in balls.iter() {
                render_swing(ball, swing.clone());
            }
            if let Some(new_swing) = new_swing {
                *mode = Mode::Swinging(new_swing);
            }
        },
        Mode::Traveling(_) => {
            for &ball in balls.iter() {
                ctx.print(
                    Point::new(2, SCREEN_HEIGHT - 3),
                    format!("Ball is Traveling {}", ball.velocity)
                );
            }
        },
        Mode::Finished => {
            ctx.print(Point::new(2, SCREEN_HEIGHT - 3), "Finishing Turn");
        },
    }
    let mut b = DrawBatch::new();
    b.draw_box(Rect::with_size(0, 43, 79, 6), ColorPair::new(WHITE, BLACK));
    b.submit(15050).expect("Box error");
    ctx.submit(20220).expect("UI Error!");
}


fn crosshair(origin: Point, degrees: &f32) -> Point {
    let radius = 20.;
    let rads = (*degrees + 90.).to_radians();
    let dx = rads.cos() * radius;
    let dy = -1. * rads.sin() * radius;
    let dir = Point::new(dx.round() as i32, dy.round() as i32);
    origin + dir
}

fn handle_swing(swing: Swing) -> Option<Swing> {
    match swing {
        Swing::Power(deg, power) => {
            let new_power = if power < 100. as f32 {
                power + 1.
            } else { power };
            if new_power >= 100. {
                Some(Swing::Accuracy(deg, new_power, 0.))
            } else {
                Some(Swing::Power(deg, new_power))
            }
        },
        _ => None
    }
}
