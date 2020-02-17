use crate::utils::CoordType;
use super::Coord;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Error {
    NotSetup(Coord),
    InvalidCoord(Coord),
    OutOfBoard,
    WrongCoordType(CoordType, CoordType),
}
