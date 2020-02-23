use std::ops::{Index, IndexMut};

/******* DevelopmentCard *******/

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DevelopmentCard {
    Knight = 0,
    RoadBuilding = 1,
    YearOfPlenty = 2,
    Monopole = 3,
    VictoryPoint = 4,
}

impl DevelopmentCard {
    pub const COUNT: usize = 5;

    pub const ALL: [DevelopmentCard; DevelopmentCard::COUNT] = [
        DevelopmentCard::Knight,
        DevelopmentCard::RoadBuilding,
        DevelopmentCard::YearOfPlenty,
        DevelopmentCard::Monopole,
        DevelopmentCard::VictoryPoint,
    ];

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/******* DevelopmentCards *******/

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DevelopmentCards {
    pub knight: u8,
    pub road_building: u8,
    pub year_of_plenty: u8,
    pub monopole: u8,
    pub victory_point: u8,
}

impl DevelopmentCards {
    pub fn new() -> DevelopmentCards {
        DevelopmentCards {
            knight: 0,
            road_building: 0,
            year_of_plenty: 0,
            monopole: 0,
            victory_point: 0,
        }
    }

    pub fn total(&self) -> u8 {
        self.knight + self.road_building + self.year_of_plenty + self.monopole + self.victory_point
    }
}

impl Index<DevelopmentCard> for DevelopmentCards {
    type Output = u8;

    fn index(&self, card: DevelopmentCard) -> &u8 {
        match card {
            DevelopmentCard::Knight => &self.knight,
            DevelopmentCard::RoadBuilding => &self.road_building,
            DevelopmentCard::YearOfPlenty => &self.year_of_plenty,
            DevelopmentCard::Monopole => &self.monopole,
            DevelopmentCard::VictoryPoint => &self.victory_point,
        }
    }
}

impl IndexMut<DevelopmentCard> for DevelopmentCards {
    fn index_mut(&mut self, card: DevelopmentCard) -> &mut u8 {
        match card {
            DevelopmentCard::Knight => &mut self.knight,
            DevelopmentCard::RoadBuilding => &mut self.road_building,
            DevelopmentCard::YearOfPlenty => &mut self.year_of_plenty,
            DevelopmentCard::Monopole => &mut self.monopole,
            DevelopmentCard::VictoryPoint => &mut self.victory_point,
        }
    }
}
