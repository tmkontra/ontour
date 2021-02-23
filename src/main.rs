use crate::prelude::*;
use bevy_ecs::{IntoSystem, Stage, WorldBuilder};
use std::iter::{FlatMap, Repeat, Zip};
use std::ops::Range;
use std::slice::Iter;

mod model;
mod systems;

mod prelude {
    pub use crate::model::*;
    pub use crate::systems::*;
    pub use crate::AppState;
    pub use bracket_lib::prelude::*;

    pub use bevy_ecs::prelude as bevy;
    pub use bevy_ecs::prelude::*;

    pub use legion::systems::CommandBuffer;
    pub use legion::systems::*;
    pub use legion::world::SubWorld;

    pub use itertools::Itertools;
    pub use itertools_num;
}

struct State {
    pub title: String,
    world: bevy::World,
    resources: bevy::Resources,
    schedule: bevy::Schedule,
}

#[derive(Copy, Clone, Debug)]
pub enum AppState {
    Menu,
    Playing,
}

impl State {
    fn build_schedule() -> bevy::Schedule {
        let mut schedule: bevy::Schedule = Default::default();
        let mut stateStage = StateStage::<AppState>::default();
        stateStage.on_state_update(AppState::Menu, menu_system::menu.system());
        stateStage.on_state_update(AppState::Playing, map_render::map_render.system());
        stateStage.on_state_update(AppState::Playing, turn_handler::turn_handler.system());
        stateStage.on_state_update(AppState::Playing, ball_render::ball_render.system());
        stateStage.on_state_update(AppState::Playing, ui_render::render_ui.system());
        schedule.add_stage("main", stateStage);
        schedule
    }

    fn new() -> Self {
        let mut world: bevy::World = Default::default();
        let mut resources: bevy::Resources = Default::default();
        let mut schedule: bevy::Schedule = State::build_schedule();

        let window = Window::new();
        let mut map = Map::load_map("src/map1.txt").unwrap();
        let ball = Ball::new(&map.tee);
        let cam = Camera::new(
            ball.tile_position(),
            map.width as i32,
            map.height as i32,
            window.width as i32 - 15,
            window.height as i32 - 10,
        );

        resources.insert(cam);
        resources.insert(FrameTime::new());
        resources.insert(bevy::State::new(AppState::Menu));
        resources.insert(map);
        resources.insert(TurnStage::start());
        resources.insert(window);
        world.spawn((ball,));

        schedule.initialize(&mut world, &mut resources);

        Self {
            title: "ON TOUR!".to_string(),
            world,
            resources,
            schedule,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.resources.insert(FrameTime::of(ctx.frame_time_ms));
        self.resources.insert(ctx.key);
        self.schedule.run(&mut self.world, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::default()
        .with_dimensions(80, 60)
        .with_font("terminal8x8.png".to_string(), 8, 8)
        .with_simple_console(80, 60, "terminal8x8.png".to_string())
        .with_title("ON TOUR")
        .with_fps_cap(30.0)
        .build()?;

    let s = State::new();

    main_loop(context, s)
}
