use crate::prelude::*;

pub fn turn_handler(key: Res<Option<VirtualKeyCode>>,
                    mut mode: ResMut<Mode>,
                    mut balls: Query<&mut Ball>
) {
    let next = mode.next();
    let new_mode = match (*mode, &next, *key) {
        (Mode::Swinging(swing), Mode::Traveling(_), Some(VirtualKeyCode::Space)) => {
            match swing {
                Swing::Start(deg) => {
                    Some(Mode::Swinging(Swing::Power(deg, 20.)))
                },
                Swing::Power(deg, pow) => {
                    Some(Mode::Swinging(Swing::Accuracy(deg, 20., 1.0)))
                },
                Swing::Accuracy(deg, pow, acc) => {
                    for mut ball in balls.iter_mut() {
                        ball.direction = deg;
                        ball.velocity = pow;
                    };
                    Some(next)
                },
            }
        },
        (Mode::Traveling(_), _, _) => {
            balls
                .iter_mut()
                .for_each(|mut b| b.tick());
            balls
                .iter_mut()
                .find(|b| b.stopped())
                .and_then(|_| Some(next))
        },
        (_, _, Some(VirtualKeyCode::Space)) => Some(next),
        _ => None
    };
    match new_mode {
        None => {},
        Some(next) => {
            *mode = next
        },
    }
}
