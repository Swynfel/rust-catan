use std::cmp::Ordering;
use std::fmt;

use super::topology::CoordTopology;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
    Void,
    Hex,
    Path,
    Intersection,
}

/// Enum representing the position of a coord on a grid
///
/// A more detailed version of [CoordType](enum.CoordType.html)
///
/// Bellow is a representation of the different position using the first letters of the variants:
///         A
///     Z       S
/// V               V
///
/// I   L   H   R   I
///
/// A               A
///     S       Z
///         V
///
pub enum DetailedType {
    /// The empty position between a hex center and it's left path
    LVoid,
    /// The empty position between a hex center and it's right path
    RVoid,
    /// The center of a hex
    OHex,
    /// The path at the top right or bottom left of a hex
    SPath,
    /// The path at the top left or bottom right of a hex
    ZPath,
    /// The vertical path at the left or right of a hex
    IPath,
    /// The intersection found bellow a hex
    VIntersection,
    /// The intersection found above a hex
    AIntersection,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coord {
    pub x: i8,
    pub y: i8,
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            v => v,
        }
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    pub const TOPOLOGY: CoordTopology = CoordTopology;

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

    pub fn get_detailed_type(&self) -> DetailedType {
        match self.get_hash() {
            (0,0) => DetailedType::OHex,
            (1,0) => DetailedType::RVoid,
            (2,0) => DetailedType::IPath,
            (3,0) => DetailedType::LVoid,
            (0,1) => DetailedType::AIntersection,
            (1,1) => DetailedType::SPath,
            (2,1) => DetailedType::VIntersection,
            (3,1) => DetailedType::ZPath,
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
