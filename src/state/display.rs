use std::fmt::Display;

use crate::utils::{Hex, LandHex, Harbor, CoordType, ToDrawType};
use crate::state::BoardState;

fn hex(hex: Option<&Hex>, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match hex {
        Some(h) => match h {
            Hex::Water => water(f),
            Hex::Land(landhex) => match landhex {
                LandHex::Desert => {
                    write!(f, " [ D ] ")
                },
                LandHex::Prod(res, v) => {
                    write!(f, " [{:>2}{}] ", v, res.to_draw_type().letter())
                },
            },
        },
        None => water(f),
    }
}

fn path(vertical: bool, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    if vertical {
        write!(f, "|.|")
    } else {
        write!(f, "=.=")
    }
}

fn intersection(harbor: Option<&Harbor>, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match harbor {
        Some(h) => match h {
            Harbor::Special(res) => {
                write!(f, "{0} {0}", res.to_draw_type().letter())
            },
            Harbor::Generic => {
                write!(f, "X X")
            }
        },
        None => write!(f, "( )"),
    }
}

fn void(f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, " ")
}

fn water(f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "  ~~~  ")
}

impl Display for BoardState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let w = self.layout.width as usize;
        for i in 0..self.layout.size {
            let coord = self.layout.coord_index(i).unwrap();
            if i % w == 0 {
                match coord.y.rem_euclid(4) {
                    0 | 1 | 3 => write!(f,"  ")?,
                    _ => (),
                };
            }
            match coord.get_type() {
                CoordType::Hex => hex(self.hexes.get_value(&coord).ok(), f),
                CoordType::Path => path(coord.y & 1 == 0, f),
                CoordType::Intersection => intersection(self.harbors.get_value(&coord).ok(), f),
                CoordType::Void => void(f),
            }?;
            if (i + 1) % w == 0 {
                writeln!(f)?;
            };
        };
        Ok(())
    }
}
