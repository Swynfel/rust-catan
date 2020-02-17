use crate::board::Error as BoardError;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Error {
    BoardError(BoardError),
}

impl From<BoardError> for Error {
    fn from(board_error: BoardError) -> Error {
        Error::BoardError(board_error)
    }
}
