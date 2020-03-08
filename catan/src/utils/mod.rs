mod development_card;
mod resource;

pub use development_card::{DevelopmentCard, DevelopmentCards};
pub use resource::{Resource, Resources};
pub use crate::board::{Coord, CoordType};
pub use crate::state::PlayerId;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Empty;

impl Empty {
    pub const INSTANCE: Empty = Empty {};
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Hex {
    Water,
    Land(LandHex),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LandHex {
    Prod(Resource, u8),
    Desert,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Harbor {
    None,
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

impl Harbor {
    pub const COUNT: usize = 6;

    pub fn to_usize(self) -> usize {
        match self {
            Harbor::None => 6,
            Harbor::Generic => 5,
            Harbor::Special(res) => {
                res as usize
            }
        }
    }
}
