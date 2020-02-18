pub mod action;
mod error;
pub mod play;
pub mod legal;

pub use error::Error;
pub use action::Action;

#[derive(Copy, Clone, Debug)]
pub enum Phase {
    InitialPlacement(u8,bool,bool), // (player,placing_second,placing_road)
    Turn(u8,bool,bool), //(player,dice_rolled,dvp_card_played)
    FinishedGame(u8), //(winning_player)
}

impl Phase {
    pub const START_GAME: Phase = Phase::InitialPlacement(0,false,false);
    pub const START_TURNS: Phase = Phase::Turn(0,false,false);
}
