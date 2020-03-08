use super::action::Action;
use crate::board::Error as BoardError;
use crate::utils::{Coord, Resource, Resources, PlayerId, DevelopmentCard};

#[derive(Copy, Clone, Debug)]
pub enum Error {
    ActionNotUnderstood,
    IncoherentAction(Action),
    IllegalAction(Action),
    ImpossibleAction(BoardError),
    IllegalTradeSameResources(Resource),
    NotEnoughResources {
        required: Resources,
        have: Resources,
    },
    NoMoreResourceInBank(Resource),
    NoMorePiece {
        piece: u8, // Road: 0, Settlement: 1, City: 2
    },
    NotConnected {
        coord: Coord,
    },
    AlreadyOccupied {
        coord: Coord,
    },
    WrongVictim {
        victim: PlayerId,
    },
    MustPickVictim,
    DevelopmentCardAlreadyPlayed,
    NoCard {
        card_type: DevelopmentCard
    },
    ThiefNotMoved {
        hex: Coord,
    }
}

impl From<BoardError> for Error {
    fn from(board_error: BoardError) -> Self {
        Error::ImpossibleAction(board_error)
    }
}
