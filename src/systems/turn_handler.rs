use crate::prelude::*;

pub fn turn_handler(
    key: Res<Option<VirtualKeyCode>>,
    mut turnStage: ResMut<TurnStage>,
    mut balls: Query<&mut Ball>,
) {
    let updatedStage: TurnStage = match *turnStage {
        TurnStage::ClubSelection(clubs, current) => {
            if let Some(VirtualKeyCode::Down) = *key {
                TurnStage::ClubSelection(clubs, clubs.next_club(current))
            } else {
                TurnStage::ClubSelection(clubs, current)
            }
        }
        TurnStage::Aiming(aim, club) => {
            let new_aim = aim.aim(*key);
            TurnStage::Aiming(Aim { degrees: new_aim }, club)
        }
        TurnStage::Swinging(swing, aim, club) => {
            let new_swing = handle_swing(swing.clone());
            if let Some(new_swing) = new_swing {
                TurnStage::Swinging(new_swing, aim, club)
            } else {
                TurnStage::Swinging(swing, aim, club)
            }
        }
        stage => stage,
    };
    let newStage = match (updatedStage, *key) {
        (TurnStage::Swinging(swing, aim, club), Some(VirtualKeyCode::Space)) => match swing {
            Swing::Start => Some(TurnStage::Swinging(
                Swing::Power(aim.degrees, 0.),
                aim,
                club,
            )),
            Swing::Power(deg, pow) => Some(TurnStage::Swinging(
                Swing::Accuracy(aim.degrees, pow, 1.0),
                aim,
                club,
            )),
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
