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
pub enum Resource {
    Brick = 0,
    Lumber = 1,
    Ore = 2,
    Grain = 3,
    Wool = 4,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CoordType {
    Void,
    Hex,
    Path,
    Intersection,
}

use std::convert::TryFrom;

impl Resource {
    pub const COUNT: usize = 5;

    pub const ALL: [Resource; Resource::COUNT] = [
        Resource::Brick,
        Resource::Lumber,
        Resource::Ore,
        Resource::Grain,
        Resource::Wool,
    ];

    pub fn letter(&self) -> char {
        match self {
            Resource::Brick => 'B',
            Resource::Lumber => 'L',
            Resource::Ore => 'O',
            Resource::Grain => 'G',
            Resource::Wool => 'W',
        }
    }
}

impl TryFrom<u8> for Resource {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Resource::Brick,
            1 => Resource::Lumber,
            2 => Resource::Ore,
            3 => Resource::Grain,
            4 => Resource::Wool,
            _ => return Err(()),
        })
    }
}
