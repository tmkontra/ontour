use crate::prelude::*;

pub fn turn_handler(
    dt: Res<FrameTime>,
    key: Res<Option<VirtualKeyCode>>,
    mut camera: ResMut<Camera>,
    mut turnStage: ResMut<TurnStage>,
    mut balls: Query<&mut Ball>,
) {
    let updatedStage: TurnStage = match *turnStage {
        TurnStage::ClubSelection(clubs, current) => match *key {
            Some(VirtualKeyCode::Down) => TurnStage::ClubSelection(clubs, clubs.next_club(current)),
            Some(VirtualKeyCode::Up) => {
                TurnStage::ClubSelection(clubs, clubs.previous_club(current))
            }
            _ => TurnStage::ClubSelection(clubs, current),
        },
        TurnStage::Aiming(aim, club) => {
            let new_aim = aim.adjust(*key);
            TurnStage::Aiming(new_aim, club)
        }
        TurnStage::Swinging(swing, aim, club) => {
            let new_swing = handle_swing(swing.clone()).unwrap_or(swing);
            TurnStage::Swinging(new_swing, aim, club)
        }
        TurnStage::Traveling(mut travel) => {
            let s = dt.seconds() * 2.;
            let dx = travel.tile_distance(s);
            println!("Frame advances {:?} ms", dt.t_ms);
            println!("Ball moves: {:?}", dx);
            balls.iter_mut().for_each(|mut b| {
                b.mv(travel.direction, dx);
                camera.update(b.tile_position());
            });

            travel.tick(s);
            TurnStage::Traveling(travel)
        }
        stage => stage,
    };
    let newStage = match (updatedStage, *key) {
        (TurnStage::Swinging(swing, aim, club), Some(VirtualKeyCode::Space)) => match swing {
            Swing::Start => Some(TurnStage::Swinging(Swing::Power(0.), aim, club)),
            Swing::Power(pow) => Some(TurnStage::Swinging(Swing::Accuracy(pow, 1.0), aim, club)),
            Swing::Accuracy(pow, acc) => Some(turnStage.next()),
        },
        (TurnStage::Traveling(mut travel), _) => {
            if travel.finished() {
                Some(turnStage.next())
            } else {
                None
            }
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
        Swing::Power(power) => {
            let new_power = if power < 100. as f32 {
                power + 1.
            } else {
                power
            };
            if new_power >= 100. {
                Some(Swing::Accuracy(new_power, 0.))
            } else {
                Some(Swing::Power(new_power))
            }
        }
        _ => None,
    }
}
