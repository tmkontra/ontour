use crate::prelude::*;

pub fn turn_handler(
    key: Res<Option<VirtualKeyCode>>,
    mut turnStage: ResMut<TurnStage>,
    mut balls: Query<&mut Ball>,
) {
    let updatedStage: TurnStage = match *turnStage {
        TurnStage::Aiming(aim) => {
            let new_aim = aim.aim(*key);
            TurnStage::Aiming(Aim { degrees: new_aim })
        }
        TurnStage::Swinging(swing) => {
            let new_swing = handle_swing(swing.clone());
            if let Some(new_swing) = new_swing {
                TurnStage::Swinging(new_swing)
            } else {
                TurnStage::Swinging(swing)
            }
        }
        stage => stage,
    };
    let newStage = match (updatedStage, *key) {
        (TurnStage::Swinging(swing), Some(VirtualKeyCode::Space)) => match swing {
            Swing::Start(deg) => Some(TurnStage::Swinging(Swing::Power(deg, 0.))),
            Swing::Power(deg, pow) => Some(TurnStage::Swinging(Swing::Accuracy(deg, pow, 1.0))),
            Swing::Accuracy(deg, pow, acc) => {
                for mut ball in balls.iter_mut() {
                    ball.direction = deg;
                    ball.velocity = pow;
                }
                Some(turnStage.next())
            }
        },
        (TurnStage::Traveling(_), _) => {
            balls.iter_mut().for_each(|mut b| b.tick());
            balls
                .iter_mut()
                .find(|b| b.stopped())
                .map(|_| turnStage.next())
        }
        (stage, Some(VirtualKeyCode::Space)) => Some(stage.next()),
        _ => None,
    };
    match newStage {
        None => *turnStage = updatedStage,
        Some(next) => *turnStage = next,
    }
}

fn handle_swing(swing: Swing) -> Option<Swing> {
    match swing {
        Swing::Power(deg, power) => {
            let new_power = if power < 100. as f32 {
                power + 1.
            } else {
                power
            };
            if new_power >= 100. {
                Some(Swing::Accuracy(deg, new_power, 0.))
            } else {
                Some(Swing::Power(deg, new_power))
            }
        }
        _ => None,
    }
}
