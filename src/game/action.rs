use crate::utils::{Coord, Resource, Resources};

//type Player = u8;

pub enum Action {
    EndTurn,
    //RollDice,
    //Discard(Resources),
    //MoveThied(Coord, Player),

    BuildRoad(Coord),
    BuildSettlement(Coord),
    BuildCity(Coord),

    TradeBank(Resource, Resource),
    //TradePlayer(Resources, Vec<bool>),
    //TradePlayerAccept,
    //TradePlayerAlternative(Resources),
    //TradePlayerDecline,

    BuyDvp,
    //DvpSoldier(Coord, Player),
    //DvpMonopole(Resource),
    //DvpProgress(Coord,Option<Coord>),
}
