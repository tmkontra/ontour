use crate::prelude::*;

pub fn hole_handler(
    hole: Res<Hole>,
    balls: Query<&Ball>,
    hole_state: Res<HoleState>,
) -> HoleState {
    match *hole_state {
        HoleState::Start => HoleState::startHole(),
        HoleState::Stroke(strokes) => {
            if let Some(_) = balls.iter().find(|b| {
                let ball_at = b.tile_position();
                hole.map.flag == ball_at
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
    mut balls: Query<&mut Ball>,
    window: Res<Window>
) {
    let next_state = match &hole_state {
        HoleState::Holed => {
            if let Some(next_hole) = course.next() {
                let next_map = &next_hole.map;
                balls.iter_mut().for_each(|mut ball| {
                    ball.move_to(&next_map.tee);
                    let cam = Camera::new(
                        ball.tile_position(),
                        next_map.width as i32,
                        next_map.height as i32,
                        window.width as i32 - 15,
                        window.height as i32 - 10,
                    );
                    commands.insert_resource(cam);
                });
                commands.insert_resource(next_hole);
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
