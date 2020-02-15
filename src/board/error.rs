use crate::utils::CoordType;
use super::Coord;

#[derive(Debug)]
pub enum Error {
    NotSetup(Coord),
    InvalidCoord(Coord),
    OutOfBoard,
    WrongCoordType(CoordType, CoordType),
}
