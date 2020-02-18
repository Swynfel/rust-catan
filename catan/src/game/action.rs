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

    BuyDvp,
    //DvpSoldier(Coord, Player),
    //DvpMonopole(Resource),
    //DvpProgress(Coord,Option<Coord>),

    Exit,
}
