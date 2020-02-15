mod resource;

pub use resource::{Resource, Resources};
pub use crate::board::{Coord, CoordType};

#[derive(Clone, PartialEq, Debug)]
pub enum Hex {
    Water,
    Land(LandHex),
}

#[derive(Clone, PartialEq, Debug)]
pub enum LandHex {
    Prod(Resource, u8),
    Desert,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Harbor {
    Generic,
    Special(Resource),
}
