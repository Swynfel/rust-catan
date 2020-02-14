use crate::constants::CoordType;

#[derive(Debug)]
pub enum BoardError {
    OutOfBoard,
    WrongCoordType(CoordType, CoordType),
}
