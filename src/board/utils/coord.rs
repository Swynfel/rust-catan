use std::fmt;

use crate::board::error::Error;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
    Void,
    Hex,
    Path,
    Intersection,
}

pub enum DetailedType {
    LVoid,
    RVoid,
    OHex,
    SPath,
    ZPath,
    IPath,
    VIntersection,
    AIntersection,
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coord {
    pub x: i8,
    pub y: i8,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coord({},{})", self.x, self.y)
    }
}

impl Coord {
    pub const ZERO: Coord = Coord::new(0, 0);

    pub const fn new(x: i8, y: i8) -> Coord {
        Coord{
            x,
            y,
        }
    }

    pub(super) fn get_hash(&self) -> (u8, u8) {
        let y_r = self.y.rem_euclid(4);
        let y_p = y_r / 2;
        let y_r = y_r % 2;
        let x_r = (self.x + 2 * y_p).rem_euclid(4);
        (x_r as u8, y_r as u8)
    }

    pub(crate) fn get_detailed_type(&self) -> DetailedType {
        match self.get_hash() {
            (0,0) => DetailedType::OHex,
            (1,0) => DetailedType::RVoid,
            (2,0) => DetailedType::IPath,
            (3,0) => DetailedType::LVoid,
            (0,1) => DetailedType::AIntersection,
            (1,1) => DetailedType::ZPath,
            (2,1) => DetailedType::VIntersection,
            (3,1) => DetailedType::SPath,
            _ => panic!("Coord has incoherent hash"),
        }
    }

    pub fn get_type(&self) -> Type {
        match self.get_hash() {
            (0,0) => Type::Hex,
            (2,0) | (1,1) | (3,1) => Type::Path,
            (0,1) | (2,1) => Type::Intersection,
            _ => Type::Void,
        }
    }

    pub fn path_intersection(&self) -> Result<[Coord; 2], Error> {
        match self.get_hash() {
            (2,0) => Ok([Coord::new(self.x, self.y - 1), Coord::new(self.x, self.y + 1)]),
            (1,1) | (3,1) => Ok([Coord::new(self.x - 1, self.y), Coord::new(self.x + 1, self.y)]),
            _ => Err(Error::InvalidCoord(*self)),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Type::Void => "V",
            Type::Hex => "H",
            Type::Path => "P",
            Type::Intersection => "I",
        })
    }
}
