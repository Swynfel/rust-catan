mod resource;
mod drawtype;

pub use resource::{Resource, Resources};
pub use drawtype::{DrawType, ToDrawType};
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

impl Hex {
    pub fn get_num(&self) -> Option<u8> {
        match self {
            Hex::Land(land) => match land {
                LandHex::Desert => None,
                LandHex::Prod(_, val) => Some(*val),
            },
            _ => None,
        }
    }
}
