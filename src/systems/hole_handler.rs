use crate::prelude::*;

pub fn hole_handler(
    map: Res<Map>,
    balls: Query<&Ball>,
    hole_state: Res<HoleState>,
) -> HoleState {
    match *hole_state {
        HoleState::Start => HoleState::startHole(),
        HoleState::Stroke(strokes) => {
            if let Some(_) = balls.iter().find(|b| {
                let ball_at = b.tile_position();
                map.flag == ball_at
            }) {
                HoleState::Holed
            } else {
                HoleState::Stroke(strokes)
            }
        }
        HoleState::Holed => HoleState::Holed
    }
}

pub fn hole_transition(
    In(hole_state): In<HoleState>,
    commands: &mut Commands,
    mut course: ResMut<Course>,
    mut balls: Query<&mut Ball>
) {
    let next_state = match &hole_state {
        HoleState::Holed => {
            if let Some(next_map) = course.next() {
                balls.iter_mut().for_each(|mut b| {
                    b.move_to(&next_map.tee)
                });
                commands.insert_resource(next_map);
                HoleState::Start
            } else {
                // TODO: score card
                panic!("Course finished!")
            }
        }
        state => *state
    };
    commands.insert_resource(next_state);
}
