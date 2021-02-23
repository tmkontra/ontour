use crate::prelude::*;

pub fn hole_handler(
    map: Res<Map>,
    balls: Query<&Ball>,
    commands: &mut Commands,
    mut hole_state: ResMut<HoleState>,
) {
    let new_state = match *hole_state {
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
        HoleState::Holed => {
            let new_hole = Map::load_map("map2.txt").unwrap();
            commands.insert_resource(new_hole);
            HoleState::startHole()
        }
    };
    *hole_state = new_state
}
