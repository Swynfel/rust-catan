use crate::utils::CoordType;
use super::Coord;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Error {
    NotSetup(Coord),
    InvalidCoord(Coord),
    OutOfBoard,
    WrongCoordType {
        expected: [bool; 4],
        received: CoordType
    },
    InvalidNeighbourTypes {
        center: CoordType,
        neighbours: CoordType
    },
}

impl Error {
    pub fn WrongCoordTypeSingle(expected: CoordType, received: CoordType) -> Error {
        let mut expected_array = [false; 4];
        expected_array[expected as usize] = true;
        Error::WrongCoordType {
            expected: expected_array,
            received,
        }
    }
}
