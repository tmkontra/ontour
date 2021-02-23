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
