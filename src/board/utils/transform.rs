use std::ops::Add;
use rand::Rng;

use super::{Coord, coord::Type};

#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum CoordRotation {
    Id = 0,
    Pos = 1,
    Pos2 = 2,
    Half = 3,
    Neg2 = 4,
    Neg = 5,
}

impl CoordRotation {
    pub const COUNT: u32 = 6;
}

impl From<u32> for CoordRotation {
    fn from(value: u32) -> Self {
        unsafe {
            std::mem::transmute(value % CoordRotation::COUNT)
        }
    }
}

impl From<i32> for CoordRotation {
    fn from(value: i32) -> Self {
        let value = value.rem_euclid(CoordRotation::COUNT as i32);
        CoordRotation::from(value as u32)
    }
}

impl Add for CoordRotation {
    type Output = CoordRotation;

    fn add(self, other: Self::Output) -> <Self as std::ops::Add<Self::Output>>::Output {
        CoordRotation::from(self as u32 + other as u32)
    }
}

pub struct CoordTransform {
    pub center: Coord,
    pub flip: bool,
    pub rot: CoordRotation,
}


impl CoordTransform {
    pub fn new(center: Coord, angle: i32, flip: bool) -> CoordTransform {
        CoordTransform {
            center,
            flip,
            rot: CoordRotation::from(angle),
        }
    }

    const FLIP_BIT: u32 = 1 << 31;

    pub fn random<R: Rng>(center: Coord, rng: &mut R) -> CoordTransform {
        let v: u32 = rng.gen();
        let flip: bool = v & CoordTransform::FLIP_BIT != 0;
        CoordTransform{
            center,
            flip,
            rot: CoordRotation::from(v),
        }
    }

    fn transform_posive_rotation(x: i8, y:i8) -> (i8, i8) {
        let mut r = (x + y) & 3;
        if r >= 2 {
            r = 2 - r;
        };
        ((x - 3 * y - r) / 2, (x + y + r) / 2)
    }

    // All possible combination of rotations and flips applied to Coord
    // Center needs to be a Hex-typed Coord, and coord can't be Void-typed Coord
    pub fn transform(&self, coord: Coord) -> Coord {
        assert_eq!(self.center.get_type(), Type::Hex);
        assert_ne!(coord.get_type(), Type::Void);
        let mut x = coord.x - self.center.x;
        let mut y = coord.y - self.center.y;
        match (self.rot, self.flip) {
            (CoordRotation::Id, false) => (),
            (CoordRotation::Id, true) => {
                y = -y;
            },
            (CoordRotation::Pos, false) => {
                let c = CoordTransform::transform_posive_rotation(x, y);
                x = c.0;
                y = c.1;
            },
            (CoordRotation::Pos, true) => {
                let c = CoordTransform::transform_posive_rotation(x, y);
                x = c.0;
                y = -c.1;
            },
            (CoordRotation::Pos2, false) => {
                let c = CoordTransform::transform_posive_rotation(-x, y);
                x = c.0;
                y = -c.1;
            },
            (CoordRotation::Pos2, true) => {
                let c = CoordTransform::transform_posive_rotation(-x, y);
                x = c.0;
                y = c.1;
            },
            (CoordRotation::Half, false) => {
                x = -x;
                y = -y;
            },
            (CoordRotation::Half, true) => {
                x = -x;
            },
            (CoordRotation::Neg2, false) => {
                let c = CoordTransform::transform_posive_rotation(-x, -y);
                x = c.0;
                y = c.1;
            },
            (CoordRotation::Neg2, true) => {
                let c = CoordTransform::transform_posive_rotation(x, y);
                x = -c.0;
                y = c.1;
            },
            (CoordRotation::Neg, false) => {
                let c = CoordTransform::transform_posive_rotation(-x, y);
                x = -c.0;
                y = c.1;
            },
            (CoordRotation::Neg, true) => {
                let c = CoordTransform::transform_posive_rotation(x, -y);
                x = c.0;
                y = c.1;
            },
        };
        Coord::new(x + self.center.x, y + self.center.y)
    }
}
