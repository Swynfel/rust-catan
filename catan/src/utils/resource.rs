use std::fmt::{self, Display};
use std::ops::{Add, Sub, AddAssign, SubAssign, Index, IndexMut};
use std::cmp::Ordering;
use std::convert::TryFrom;

/******* Resource *******/

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

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Resource::Brick => write!(f, "B"),
            Resource::Lumber => write!(f, "L"),
            Resource::Ore => write!(f, "O"),
            Resource::Grain => write!(f, "G"),
            Resource::Wool => write!(f, "W"),
        }?;
        Ok(())
    }
}

/******* Resources *******/

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
    pub const ROAD: Resources = Resources::new(1,1,0,0,0);
    pub const SETTLEMENT: Resources = Resources::new(1,1,0,1,1);
    pub const CITY: Resources = Resources::new(0,0,3,2,0);
    pub const DVP_CARD: Resources = Resources::new(0,0,1,1,1);

    pub const fn new(brick: i8, lumber: i8, ore :i8, grain: i8, wool: i8) -> Self {
        Resources {
            brick,
            lumber,
            ore,
            grain,
            wool
        }
    }

    pub fn new_one(resource: Resource, quantity: i8) -> Self {
        match resource {
            Resource::Brick => Resources::new(quantity, 0, 0, 0, 0),
            Resource::Lumber => Resources::new(0, quantity, 0, 0, 0),
            Resource::Ore => Resources::new(0, 0, quantity, 0, 0),
            Resource::Grain => Resources::new(0, 0, 0, quantity, 0),
            Resource::Wool => Resources::new(0, 0, 0, 0, quantity),
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

impl Sub for Resources {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Resources::new(
            self.brick - other.brick,
            self.lumber - other.lumber,
            self.ore - other.ore,
            self.grain - other.grain,
            self.wool - other.wool,
        )
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, other: Self) {
        self.brick += other.brick;
        self.lumber += other.lumber;
        self.ore += other.ore;
        self.grain += other.grain;
        self.wool += other.wool;
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, other: Self) {
        self.brick -= other.brick;
        self.lumber -= other.lumber;
        self.ore -= other.ore;
        self.grain -= other.grain;
        self.wool -= other.wool;
    }
}

impl Index<Resource> for Resources {
    type Output = i8;

    fn index(&self, resource: Resource) -> &i8 {
        match resource {
            Resource::Brick => &self.brick,
            Resource::Lumber => &self.lumber,
            Resource::Ore => &self.ore,
            Resource::Grain => &self.grain,
            Resource::Wool => &self.wool,
        }
    }
}

impl IndexMut<Resource> for Resources {
    fn index_mut(&mut self, resource: Resource) -> &mut i8 {
        match resource {
            Resource::Brick => &mut self.brick,
            Resource::Lumber => &mut self.lumber,
            Resource::Ore => &mut self.ore,
            Resource::Grain => &mut self.grain,
            Resource::Wool => &mut self.wool,
        }
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
