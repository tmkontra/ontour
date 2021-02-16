use crate::prelude::*;
use bevy_ecs::{WorldBuilder, IntoSystem, Stage};

mod map;
mod tile;
mod ball;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::map::*;
    pub use crate::ball::*;
    pub use crate::tile::*;
    pub use crate::systems::*;
    pub use crate::Mode;
    pub use crate::Swing;
    pub use crate::Aim;

    pub const FRAME_DURATION: f32 = 300.;
    pub const SCREEN_HEIGHT: u8 = 50;
    pub const SCREEN_WIDTH: u8 = 80;

    pub use bevy_ecs::prelude as bevy;
    pub use bevy_ecs::prelude::*;

    pub use legion::systems::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
}

struct State {
    pub message: String,
    SCREEN_HEIGHT: u8,
    SCREEN_WIDTH: u8,
    pub map: Map,
    world: bevy::World,
    resources: bevy::Resources,
    schedule: bevy::Schedule

}

#[derive(Copy, Clone, Debug)]
pub struct Aim {
    degrees: f32
}

impl Aim {
    const RATE: f32 = 3.;

    pub fn new() -> Self {
        Self { degrees: 0. }
    }

    fn aim(&self, key: Option<VirtualKeyCode>) -> f32 {
        match key {
            Some(VirtualKeyCode::Left) => {
                self.degrees + Aim::RATE
            },
            Some(VirtualKeyCode::Right) => {
                self.degrees - Aim::RATE
            },
            _ => {
                self.degrees
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Swing {
    Start(f32),
    Power(f32, f32),
    Accuracy(f32, f32, f32)
}

impl Swing {
    pub fn direction(&self) -> &f32 {
        match self {
            Swing::Start(deg) => deg,
            Swing::Power(deg, _) => deg,
            Swing::Accuracy(deg, _, _) => deg,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Travel {

}

#[derive(Copy, Clone, Debug)]
struct Finished {

}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Aiming(Aim),
    Swinging(Swing),
    Traveling(Travel),
    Finished
}

impl Mode {
    pub fn default() -> Mode {
        Mode::Aiming(Aim::new())
    }

    fn start_swing(degrees: f32) -> Mode {
        Mode::Swinging(Swing::Start(degrees))
    }

    pub fn next(&self) -> Mode {
        match self {
            Mode::Aiming(Aim { degrees }) => Mode::start_swing(degrees.clone()),
            Mode::Swinging(_) => Mode::Traveling(Travel{}),
            _ => Mode::Finished
        }
    }
}

impl State {
    fn build_schedule() -> bevy::Schedule {
        let mut schedule: bevy::Schedule = Default::default();
        schedule.add_stage("main", SystemStage::parallel());
        schedule.add_system_to_stage("main", turn::turn.system());
        schedule.add_system_to_stage("main", map_render::map_render.system());
        schedule.add_system_to_stage("main", ball_render::ball_render.system());
        schedule.add_system_to_stage("main", ui_render::render_ui.system());
        schedule
    }

    fn new() -> Self {
        let mut world: bevy::World = Default::default();
        let mut resources: bevy::Resources = Default::default();
        let mut schedule: bevy::Schedule = State::build_schedule();

        let mut map = Map::load_map(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "src/map1.txt"
        ).unwrap();
        let ball = Ball::new(&map.tee);

        resources.insert(map.clone());
        resources.insert(Mode::default());
        world.spawn((ball,));

        schedule.initialize(&mut world, &mut resources);

        Self {
            message: "ON TOUR!".to_string(),
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            map,
            world,
            resources,
            schedule
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.resources.insert(ctx.key);
        self.schedule.run(&mut self.world, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("ON TOUR")
        .with_fps_cap(30.0)
        .build()?;

    let s = State::new();

    main_loop(context, s)
}
