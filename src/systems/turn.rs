use crate::prelude::*;

#[system]
pub fn turn(#[resource] key: &Option<VirtualKeyCode>,
              #[resource] mode: &mut Mode
) {
    let next = mode.next();
    let new_mode = match (&mode, &next, key) {
        (Mode::Swinging(swing), Mode::Traveling(_), Some(VirtualKeyCode::Space)) => {
            match swing {
                Swing::Start(deg) => {
                    Some(Mode::Swinging(Swing::Power(*deg, 20.)))
                },
                Swing::Power(deg, pow) => {
                    Some(Mode::Swinging(Swing::Accuracy(*deg, 20., 1.0)))
                },
                Swing::Accuracy(deg, pow, acc) => {
                    // self.ball.direction = *deg;
                    // self.ball.velocity = *pow;
                    Some(next)
                },
            }
        },
        (Mode::Traveling(_), _, _) => {
            // if self.ball.stopped() {
            //     self.mode = next
            // }
            Some(next)
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
