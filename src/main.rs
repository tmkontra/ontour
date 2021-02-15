use crate::prelude::*;

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

    pub const FRAME_DURATION: f32 = 300.;
    pub const SCREEN_HEIGHT: u8 = 50;
    pub const SCREEN_WIDTH: u8 = 80;

    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
}

struct State {
    pub message: String,
    SCREEN_HEIGHT: u8,
    SCREEN_WIDTH: u8,
    pub map: Map,
    ecs: World,
    resources: Resources,
    systems: Schedule

}

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

#[derive(Clone)]
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

pub struct Travel {

}

struct Finished {

}

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
    fn build_scheduler() -> Schedule {
        Schedule::builder()
            .add_system(map_render::map_render_system())
            .add_system(ball_render::ball_render_system())
            .add_system(turn::turn_system())
            .build()
    }

    fn new() -> Self {
        let mut map = Map::load_map(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "src/map1.txt"
        ).unwrap();
        let ball = Ball::new(&map.tee);
        let mut ecs = World::default();
        let mut res = Resources::default();
        res.insert(map.clone());
        res.insert(Mode::default());
        &ecs.push((ball,));
        Self {
            message: "ON TOUR!".to_string(),
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            map,
            ecs,
            resources: res,
            systems: State::build_scheduler()
        }
    }

    fn crosshair(origin: Point, degrees: &f32) -> Point {
        let radius = 20.;
        let rads = (*degrees + 90.).to_radians();
        let dx = rads.cos() * radius;
        let dy = -1. * rads.sin() * radius;
        let dir = Point::new(dx.round() as i32, dy.round() as i32);
        origin + dir
    }

    fn handle_swing(&self, swing: Swing) -> Option<Swing> {
        match swing {
            Swing::Power(deg, power) => {
                let new_power = if power < 100. as f32 {
                    power + 1.
                } else { power };
                if new_power >= 100. {
                    Some(Swing::Accuracy(deg, new_power, 0.))
                } else {
                    Some(Swing::Power(deg, new_power))
                }
            },
            _ => None
        }
    }

    fn render_swing(&self, swing: Swing, ctx: &mut BTerm) {
        let p = Point::new(27, 30);
        match swing {
            Swing::Start(deg) => {
                ctx.print(2, self.SCREEN_HEIGHT - 3, "[Start] Aim, Press Space to Start Swing!");
                let coord  = State::crosshair(p, swing.direction());
                let bg = self.map.bg(coord);
                ctx.set(coord.x, coord.y, WHITE, bg, 9);
            },
            Swing::Power(deg, pow) => {
                ctx.print(2, self.SCREEN_HEIGHT - 3, "[Power] Aim, Press Space to Start Swing!");
                let coord  = State::crosshair(p, swing.direction());
                let bg = self.map.bg(coord);
                ctx.set(coord.x, coord.y, WHITE, bg, 9);
                ctx.draw_bar_horizontal(2, self.SCREEN_HEIGHT - 10,
                                        51,
                                        pow as i32,
                                        100,
                                        RED, BLACK);
            },
            Swing::Accuracy(_, _, _) => {
                ctx.print(2, self.SCREEN_HEIGHT - 3, "[Acc] Aim, Press Space to Start Swing!");
                let coord  = State::crosshair(p, swing.direction());
                let bg = self.map.bg(coord);
                ctx.set(coord.x, coord.y, WHITE, bg, 9);
            },
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // match &self.mode {
        //     Mode::Aiming(aim) => {
        //         ctx.print(2, self.SCREEN_HEIGHT - 3, "Aiming");
        //         let new_aim = aim.aim(ctx.key);
        //         self.mode = Mode::Aiming(Aim{ degrees: new_aim})
        //     },
        //     Mode::Swinging(swing) => {
        //         let new_swing = self.handle_swing(swing.clone());
        //         self.render_swing(swing.clone(), ctx);
        //         if let Some(new_swing) = new_swing {
        //             self.mode = Mode::Swinging(new_swing);
        //         }
        //     },
        //     Mode::Traveling(_) => {
        //         ctx.print(
        //             2,
        //             self.SCREEN_HEIGHT - 3,
        //             format!("Ball is Traveling {}", 100.)
        //         )
        //     },
        //     Mode::Finished => {
        //         ctx.print(2, self.SCREEN_HEIGHT - 3, "Finishing Turn")
        //     },
        // }
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
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
