use crate::prelude::*;
use bevy_ecs::{IntoSystem, Stage, WorldBuilder};
use std::ops::Range;

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
    pub use crate::Camera;
    pub use crate::Club;
    pub use crate::FrameTime;
    pub use crate::Swing;
    pub use crate::TurnStage;
    pub use crate::Window;
    pub use bracket_lib::prelude::*;

    pub use bevy_ecs::prelude as bevy;
    pub use bevy_ecs::prelude::*;

    pub use legion::systems::CommandBuffer;
    pub use legion::systems::*;
    pub use legion::world::SubWorld;

    pub use itertools_num;
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

pub struct FrameTime {
    pub t_ms: f32,
}

impl FrameTime {
    pub fn new() -> Self {
        FrameTime { t_ms: 0. }
    }

    pub fn of(ms: f32) -> Self {
        FrameTime { t_ms: ms }
    }

    pub fn seconds(&self) -> f32 {
        self.t_ms / 1000.
    }
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
    pub loft_deg: f32,
    pub max_initial_velocity: f32,
}

impl Club {
    const DRIVER: Club = Club {
        id: 1,
        name: "Driver",
        loft_deg: 12.,
        max_initial_velocity: 73.76,
    };
    const PUTTER: Club = Club {
        id: 2,
        name: "Putter",
        loft_deg: 0.,
        max_initial_velocity: 1.,
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
    Power(f32),
    Accuracy(f32, f32),
}

impl Swing {}

#[derive(Copy, Clone, Debug)]
pub struct Travel {
    pub direction: f32,
    initial_velocity: f32,
    fx: f32,
    fy: f32,
    velocity_x: f32,
    velocity_y: f32,
    ax: f32,
    ay: f32,
    sy: f32,
    lift_mag: f32,
    t_elapsed: f32,
}

/** TODO: implement ground travel after carry **/
impl Travel {
    const g: f32 = -9.81;
    const CD: f32 = 0.2;
    const RPM: f32 = 3275.;
    const rho: f32 = 1.225;
    const area: f32 = 0.00138;
    const sf: f32 = -0.00026;
    const lf: f32 = 0.285;
    const mass: f32 = 0.045;
    const meters_per_tile: f32 = 8.33333;

    fn drag(v: f32) -> f32 {
        -0.5 * Travel::rho * (v.powf(2.)) * Travel::CD * Travel::area
    }

    fn meters_to_tile_distance(meters: f32) -> f32 {
        meters / Travel::meters_per_tile
    }

    pub fn new(power: &f32, aim: &Aim, club: &Club) -> Self {
        let lift_mag: f32 = Travel::lf * (1. - (Travel::sf * Travel::RPM).exp());
        let theta_rad = (club.loft_deg).to_radians();
        let fx = theta_rad.cos();
        let fy = theta_rad.sin();
        let vi = *power / 100. * club.max_initial_velocity;
        let vx = vi * fx;
        let vy = vi * fy;
        let ax = Travel::drag(vx) / Travel::mass;
        let ay = (Travel::drag(vy) / Travel::mass) + Travel::g;
        Travel {
            direction: aim.degrees,
            initial_velocity: vi,
            fx,
            fy,
            velocity_x: vx,
            velocity_y: vy,
            ax,
            ay,
            sy: 0.,
            lift_mag,
            t_elapsed: 0.,
        }
    }

    pub fn finished(&self) -> bool {
        self.sy < 0.
    }

    pub fn tile_distance(&self, dt: f32) -> f32 {
        let meters = self.velocity_x * dt + 0.5 * self.ax * dt.powf(2.);
        Travel::meters_to_tile_distance(meters)
    }

    pub fn tick(&mut self, dt: f32) {
        let sy: f32 = self.sy + self.velocity_y * dt + (0.5 * self.ay * dt.powf(2.));
        let vx = self.velocity_x + self.ax * dt;
        let vy = self.velocity_y + self.ay * dt;
        let theta_i = (vy / vx).atan();
        let lx = self.lift_mag * theta_i.sin();
        let ly = self.lift_mag * theta_i.cos();
        let ax = Travel::drag(vx) / Travel::mass + (lx / Travel::mass);
        let ay = Travel::drag(vy) / Travel::mass + Travel::g + (ly / Travel::mass);
        self.sy = sy;
        println!("Ball at height: {:?}", sy);
        self.velocity_x = vx;
        self.velocity_y = vy;
        self.ax = ax;
        self.ay = ay;
        self.t_elapsed += dt;
    }
}

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
            TurnStage::Swinging(swing, aim, club) => match swing {
                Swing::Accuracy(pow, acc) => TurnStage::Traveling(Travel::new(pow, aim, club)),
                _ => panic!("Cannot transition from swing to travel yet!"),
            },
            TurnStage::Traveling(_) => TurnStage::Finished,
            TurnStage::Finished => TurnStage::start(),
        }
    }
}

pub struct Camera {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    map_x: i32,
    map_y: i32,
    display_width: i32,
    display_height: i32,
}

impl Camera {
    pub fn new(position: Point, map_x: i32, map_y: i32, display_width: i32, display_height: i32) -> Self {
        let y = if map_y - position.y < display_height / 2 {
            let dy = (display_height / 2) - (map_y - position.y);
            position.y - dy
        } else if position.y < display_height / 2 {
            display_height / 2
        } else {
            position.y
        };
        let x = if map_x - position.x < display_width / 2 {
            let dx = (display_width / 2) - (map_x - position.x);
            position.x - dx
        } else if position.x < display_width / 2 {
            display_width / 2
        } else {
            position.x
        };
        Self {
            display_width,
            display_height,
            map_x,
            map_y,
            min_x: x - display_width / 2,
            max_x: x + display_width / 2,
            min_y: y - display_height / 2,
            max_y: y + display_height / 2,
        }
    }

    pub fn width(&self) -> i32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> i32 {
        self.max_y - self.min_y
    }

    pub fn y_iter(&self) -> Range<i32> {
        self.min_y..self.max_y - 2
    }

    pub fn x_iter(&self) -> Range<i32> {
        self.min_x..self.max_x - 2
    }

    pub fn update(&mut self, position: Point) {
        let y = if self.map_y - position.y < self.display_height / 2 {
            let dy = (self.display_height / 2) - (self.map_y - position.y);
            position.y - dy
        } else if position.y < self.display_height / 2 {
            self.display_height / 2
        } else {
            position.y
        };
        let x = if self.map_x - position.x < self.display_width / 2 {
            let dx = (self.display_width / 2) - (self.map_x - position.x);
            position.x - dx
        } else if position.x < self.display_width / 2 {
            self.display_width / 2
        } else {
            position.x
        };
        self.min_x = x - self.display_width / 2;
        self.max_x = x + self.display_width / 2;
        self.min_y = y - self.display_height / 2;
        self.max_y = y + self.display_height / 2;
    }

    pub fn render_coordinate(&self, position: Point) -> Point {
        Point::new(position.x - self.min_x + 1, position.y - self.min_y + 1)
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
