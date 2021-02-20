use crate::prelude::*;
use bevy_ecs::{IntoSystem, Stage, WorldBuilder};

mod ball;
mod map;
mod systems;
mod tile;

mod prelude {
    pub use crate::ball::*;
    pub use crate::map::*;
    pub use crate::systems::*;
    pub use crate::tile::*;
    pub use crate::Aim;
    pub use crate::AppState;
    pub use crate::Club;
    pub use crate::Swing;
    pub use crate::TurnStage;
    pub use crate::Window;
    pub use bracket_lib::prelude::*;

    pub use bevy_ecs::prelude as bevy;
    pub use bevy_ecs::prelude::*;

    pub use legion::systems::CommandBuffer;
    pub use legion::systems::*;
    pub use legion::world::SubWorld;
}

struct State {
    pub title: String,
    world: bevy::World,
    resources: bevy::Resources,
    schedule: bevy::Schedule,
}

pub struct Window {
    pub height: u8,
    pub width: u8,
}

impl Window {
    const SCREEN_HEIGHT: u8 = 60;
    const SCREEN_WIDTH: u8 = 80;

    pub fn new() -> Self {
        Self {
            height: Window::SCREEN_HEIGHT,
            width: Window::SCREEN_WIDTH,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AppState {
    Menu,
    Playing,
}

#[derive(Copy, Clone, Debug)]
pub struct Aim {
    degrees: f32,
}

impl Aim {
    const RATE: f32 = 3.;

    pub fn new() -> Self {
        Self { degrees: 0. }
    }

    fn aim(&self, key: Option<VirtualKeyCode>) -> f32 {
        match key {
            Some(VirtualKeyCode::Left) => self.degrees + Aim::RATE,
            Some(VirtualKeyCode::Right) => self.degrees - Aim::RATE,
            _ => self.degrees,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Club {
    id: u32,
    pub name: &'static str,
    // loft, speed, accuracy
}

impl Club {
    const DRIVER: Club = Club {
        id: 1,
        name: "Driver",
    };
    const PUTTER: Club = Club {
        id: 2,
        name: "Putter",
    };

    pub fn default() -> Self {
        Club::DRIVER
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ClubSet {
    clubs: [Club; 2],
}

impl ClubSet {
    pub fn previous_club(&self, selected: usize) -> usize {
        if selected == 0 {
            self.clubs.len() - 1
        } else {
            selected - 1
        }
    }

    pub fn next_club(&self, selected: usize) -> usize {
        self.clubs.get(selected + 1).map_or(0, |_| selected + 1)
    }

    pub fn at(&self, selection: &usize) -> Club {
        self.clubs[*selection]
    }

    pub fn default() -> ClubSet {
        ClubSet {
            clubs: [Club::DRIVER, Club::PUTTER],
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Swing {
    Start,
    Power(f32, f32),
    Accuracy(f32, f32, f32),
}

impl Swing {
    pub fn direction(&self) -> &f32 {
        match self {
            Swing::Start => &0.,
            Swing::Power(deg, _) => deg,
            Swing::Accuracy(deg, _, _) => deg,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Travel {}

#[derive(Copy, Clone, Debug)]
struct Finished {}

#[derive(Copy, Clone, Debug)]
pub enum TurnStage {
    ClubSelection(ClubSet, usize),
    Aiming(Aim, Club),
    Swinging(Swing, Aim, Club),
    Traveling(Travel),
    Finished,
}

impl TurnStage {
    pub fn start() -> TurnStage {
        let set = ClubSet::default();
        TurnStage::ClubSelection(set, 0)
    }

    fn start_swing(aim: Aim, club: Club) -> TurnStage {
        TurnStage::Swinging(Swing::Start, aim, club)
    }

    pub fn next(&self) -> TurnStage {
        match self {
            TurnStage::ClubSelection(clubs, club) => TurnStage::Aiming(Aim::new(), clubs.at(club)),
            TurnStage::Aiming(aim, club) => TurnStage::start_swing(aim.clone(), club.clone()),
            TurnStage::Swinging(_, _, _) => TurnStage::Traveling(Travel {}),
            TurnStage::Traveling(_) => TurnStage::Finished,
            TurnStage::Finished => TurnStage::start(),
        }
    }
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
        let mut map = Map::load_map(window.width - 15, window.height - 10, "src/map1.txt").unwrap();
        let ball = Ball::new(&map.tee);

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
