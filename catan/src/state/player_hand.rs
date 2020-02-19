use std::ops::{Index, IndexMut};
use crate::utils::{Resource, Resources, Harbor};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DvpCardHand {
    pub knigth: u8,
    pub road_building: u8,
    pub year_of_plenty: u8,
    pub monopole: u8,
    pub victory_point: u8,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AccessibleHarbor {
    harbors: [bool; Harbor::COUNT],
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PlayerHand {
    pub resources: Resources,
    pub road_pieces: u8,
    pub settlement_pieces: u8,
    pub city_pieces: u8,
    pub building_vp: u8,
    pub knights: u8,
    pub continous_road: u8,
    pub dvp_cards: DvpCardHand,
    pub harbor: AccessibleHarbor,
}

impl AccessibleHarbor {
    pub fn new() -> AccessibleHarbor {
        AccessibleHarbor {
            harbors: [false; Harbor::COUNT],
        }
    }

    pub fn rate(&self, resource: Resource) -> u8 {
        let mut required = 4;
        if self[Harbor::Special(resource)] {
            required = 2;
        } else if self[Harbor::Generic] {
            required = 3;
        }
        required
    }
}

impl Index<Harbor> for AccessibleHarbor {
    type Output = bool;

    fn index(&self, harbor: Harbor) -> &bool {
         &self.harbors[harbor.to_usize()]
    }
}

impl IndexMut<Harbor> for AccessibleHarbor {
    fn index_mut(&mut self, harbor: Harbor) -> &mut bool {
         &mut self.harbors[harbor.to_usize()]
    }
}

impl DvpCardHand {
    pub fn new() -> DvpCardHand {
        DvpCardHand {
            knigth: 0,
            road_building: 0,
            year_of_plenty: 0,
            monopole: 0,
            victory_point: 0,
        }
    }

    pub fn total(&self) -> u8 {
        self.knigth + self.road_building + self.year_of_plenty + self.monopole + self.victory_point
    }
}

impl PlayerHand {
    pub(super) fn new() -> PlayerHand {
        PlayerHand {
            resources: Resources::ZERO,
            road_pieces: 15,
            settlement_pieces: 5,
            city_pieces: 4,
            building_vp: 0,
            knights: 0,
            continous_road: 0,
            dvp_cards: DvpCardHand::new(),
            harbor: AccessibleHarbor::new(),
        }
    }
}
