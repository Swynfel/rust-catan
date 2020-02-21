use super::action::Action;
use crate::board::Error as BoardError;
use crate::utils::{Resource, Resources};

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
    }
}

impl From<BoardError> for Error {
    fn from(board_error: BoardError) -> Self {
        Error::ImpossibleAction(board_error)
    }
}
