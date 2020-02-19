use crate::utils::CoordType;
use super::Coord;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Error {
    NotSetup(Coord),
    InvalidCoord(Coord),
    OutOfBoard,
    WrongCoordType {
        expected: CoordType,
        received: CoordType,
    },
    MultiWrongCoordType {
        expected: [bool; 4],
        received: CoordType,
    },
    InvalidNeighbourTypes {
        center: CoordType,
        neighbours: CoordType
    },
}
