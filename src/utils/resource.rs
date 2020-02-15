use std::cmp::Ordering;
use std::ops::Add;
use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Resource {
    Brick = 0,
    Lumber = 1,
    Ore = 2,
    Grain = 3,
    Wool = 4,
}

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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Resources{
    brick: i8,
    lumber: i8,
    ore: i8,
    grain: i8,
    wool: i8,
}

impl Resources {
    pub const ZERO: Resources = Resources::new(0,0,0,0,0);

    pub const fn new(brick: i8, lumber: i8, ore :i8, grain: i8, wool: i8) -> Self {
        Resources {
            brick,
            lumber,
            ore,
            grain,
            wool
        }
    }

    fn cmp(&self, other: &Resources) -> [Ordering; Resource::COUNT] {[
        self.brick.cmp(&other.brick),
        self.lumber.cmp(&other.lumber),
        self.ore.cmp(&other.ore),
        self.grain.cmp(&other.grain),
        self.wool.cmp(&other.wool),
    ]}

    pub fn valid_trade(&self) -> bool {
        !(self >= &Resources::ZERO) && !(self <= &Resources::ZERO)
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Resources::new(
            self.brick + other.brick,
            self.lumber + other.lumber,
            self.ore + other.ore,
            self.grain + other.grain,
            self.wool + other.wool,
        )
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Resources) -> Option<Ordering> {
        let orderings = self.cmp(other);
        let mut current = Ordering::Equal;
        for ordering in orderings.iter() {
            match ordering {
                Ordering::Equal => (),
                Ordering::Greater => {
                    if current == Ordering::Less {
                        return None;
                    }
                    current = Ordering::Greater;
                }
                Ordering::Less => {
                    if current == Ordering::Greater {
                        return None;
                    }
                    current = Ordering::Less;

                }
            }
        };
        Some(current)
    }
}
