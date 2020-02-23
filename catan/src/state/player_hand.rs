use std::ops::{Index, IndexMut};
use crate::utils::{Resource, Resources, Harbor, DevelopmentCards};

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
    pub development_cards: DevelopmentCards,
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

    pub fn add(&mut self, harbor: Harbor) {
        match harbor {
            Harbor::None => return,
            _ => (),
        }
        self[harbor] = true;
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
            development_cards: DevelopmentCards::new(),
            harbor: AccessibleHarbor::new(),
        }
    }
}
