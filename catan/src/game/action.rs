use crate::utils::{Coord, Resource, Resources, PlayerId};

//typeCatanPlayer= u8;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Action {
    EndTurn,
    RollDice,
    //Discard(Resources),
    MoveThief {
        hex: Coord,
        victim: PlayerId,
    },

    BuildRoad {
        path: Coord
    },
    BuildSettlement {
        intersection: Coord
    },
    BuildCity {
        intersection: Coord
    },

    TradeBank {
        given: Resource,
        asked: Resource
    },
    //TradePlayer(Resources, Vec<bool>),
    //TradePlayerAccept,
    //TradePlayerAlternative(Resources),
    //TradePlayerDecline,

    BuyDevelopment,
    DevelopmentKnight,
    DevelopmentRoadBuilding,
    DevelopmentYearOfPlenty,
    ChooseFreeResource {
        resource: Resource
    },
    DevelopmentMonopole {
        resource: Resource,
    },
    Exit,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ActionCategory {
    EndTurn = 0,
    RollDice = 1,
    MoveThief = 2,
    BuildRoad = 3,
    BuildSettlement = 4,
    BuildCity = 5,
    TradeBank = 6,
    BuyDevelopment = 7,
    DevelopmentKnight = 8,
    DevelopmentRoadBuilding = 9,
    DevelopmentYearOfPlenty = 10,
    ChooseFreeResource = 11,
    DevelopmentMonopole = 12,
    Exit = 13,
}

impl Action {
    pub fn category(&self) -> ActionCategory {
        match self {
            Action::EndTurn => ActionCategory::EndTurn,
            Action::RollDice => ActionCategory::RollDice,
            Action::MoveThief { hex: _, victim: _ }=> ActionCategory::MoveThief,
            Action::BuildRoad { path: _ } => ActionCategory::BuildRoad,
            Action::BuildSettlement { intersection: _ } => ActionCategory::BuildSettlement,
            Action::BuildCity { intersection: _ } => ActionCategory::BuildCity,
            Action::TradeBank { given: _, asked: _ } => ActionCategory::TradeBank,
            Action::BuyDevelopment => ActionCategory::BuyDevelopment,
            Action::DevelopmentKnight => ActionCategory::DevelopmentKnight,
            Action::DevelopmentRoadBuilding  => ActionCategory::DevelopmentRoadBuilding,
            Action::DevelopmentYearOfPlenty => ActionCategory::DevelopmentYearOfPlenty,
            Action::ChooseFreeResource { resource: _ } => ActionCategory::ChooseFreeResource,
            Action::DevelopmentMonopole { resource: _ }  => ActionCategory::DevelopmentMonopole,
            Action::Exit => ActionCategory::Exit,
        }
    }
}

impl ActionCategory {
    pub const COUNT: usize = 14;
}
