use super::action::Action;
use crate::board::Error as BoardError;

#[derive(Copy, Clone, Debug)]
pub enum Error {
    IncoherentAction(Action),
    IllegalAction(Action),
    ImpossibleAction(BoardError),
}

impl From<BoardError> for Error {
    fn from(board_error: BoardError) -> Self {
        Error::ImpossibleAction(board_error)
    }
}
