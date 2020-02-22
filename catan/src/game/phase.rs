use crate::state::PlayerId;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Phase {
    InitialPlacement(PlayerId,bool,bool), // (player,placing_second,placing_road)
    Turn(PlayerId,bool,bool), //(player,dice_rolled,dvp_card_played)
    FinishedGame(PlayerId), //(winning_player)
}

impl Phase {
    pub const START_GAME: Phase = Phase::InitialPlacement(PlayerId::FIRST,false,false);
    pub const START_TURNS: Phase = Phase::Turn(PlayerId::FIRST,false,false);

    pub fn player(&self) -> PlayerId {
        match self {
            Phase::InitialPlacement(player, _, _) => *player,
            Phase::Turn(player, _, _) => *player,
            Phase::FinishedGame(player) => *player,
        }
    }
    pub fn is_turn(&self) -> bool {
        if let Phase::Turn(_,_,_) = self {
            true
        } else {
            false
        }
    }
}
