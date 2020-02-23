use crate::utils::{Coord, Resource, Resources};

//type Player = u8;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Action {
    EndTurn,
    RollDice,
    //Discard(Resources),
    //MoveThied(Coord, Player),

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
    //DevelopmentSoldier(Coord, Player),
    //DevelopmentMonopole(Resource),
    //DevelopmentProgress(Coord,Option<Coord>),

    Exit,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ActionCategory {
    EndTurn = 0,
    RollDice = 1,
    BuildRoad = 2,
    BuildSettlement = 3,
    BuildCity = 4,
    TradeBank = 5,
    BuyDevelopment = 6,
    Exit = 7,
}

impl Action {
    pub fn category(&self) -> ActionCategory {
        match self {
            Action::EndTurn => ActionCategory::EndTurn,
            Action::RollDice => ActionCategory::RollDice,
            Action::BuildRoad { path: _ } => ActionCategory::BuildRoad,
            Action::BuildSettlement { intersection: _ } => ActionCategory::BuildSettlement,
            Action::BuildCity { intersection: _ } => ActionCategory::BuildCity,
            Action::TradeBank { given: _, asked: _ } => ActionCategory::TradeBank,
            Action::BuyDevelopment => ActionCategory::BuyDevelopment,
            Action::Exit => ActionCategory::Exit,
        }
    }
}

impl ActionCategory {
    pub const COUNT: usize = 8;
}
