use crate::prelude::*;
use club::Club;

#[derive(Copy, Clone, Debug)]
pub struct Aim {
    pub degrees: f32,
}

impl Aim {
    const RATE: f32 = 3.;

    pub fn new() -> Self {
        Self { degrees: 0. }
    }

    pub fn of(degrees: f32) -> Self {
        Self { degrees }
    }

    pub fn adjust(&self, key: Option<VirtualKeyCode>) -> Aim {
        let deg = match key {
            Some(VirtualKeyCode::Left) => self.degrees + Aim::RATE,
            Some(VirtualKeyCode::Right) => self.degrees - Aim::RATE,
            _ => self.degrees,
        };
        Aim::of(deg)
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
                accuracy => TurnStage::Swinging(swing.clone(), aim.clone(), club.clone()),
            },
            TurnStage::Traveling(_) => TurnStage::Finished,
            TurnStage::Finished => TurnStage::start(),
        }
    }
}
