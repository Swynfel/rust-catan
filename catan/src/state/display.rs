use std::fmt::Display;

use crate::utils::{Hex, LandHex, Harbor, CoordType};
use crate::state::State;

fn hex(hex: Hex, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match hex {
        Hex::Water => water(f),
        Hex::Land(landhex) => match landhex {
            LandHex::Desert => {
                write!(f, " [ D ] ")
            },
            LandHex::Prod(res, v) => {
                write!(f, " [{:>2}{}] ", v, res)
            },
        },
    }
}

fn path(vertical: bool, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    if vertical {
        write!(f, "|.|")
    } else {
        write!(f, "=.=")
    }
}

fn intersection(harbor: Harbor, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match harbor {
        Harbor::Special(res) => write!(f, "{0} {0}", res),
        Harbor::Generic => write!(f, "X X"),
        Harbor::None => write!(f, "( )"),
    }
}

fn void(f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, " ")
}

fn water(f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "  ~~~  ")
}

impl Display for dyn State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let w = self.get_layout().width as usize;
        for i in 0..self.get_layout().size {
            let coord = self.get_layout().coord_index(i).unwrap();
            if i % w == 0 {
                match coord.y.rem_euclid(4) {
                    0 | 1 | 3 => write!(f,"  ")?,
                    _ => (),
                };
            }
            match coord.get_type() {
                CoordType::Hex => hex(self.get_static_hex(coord).unwrap(), f),
                CoordType::Path => path(coord.y & 1 == 0, f),
                CoordType::Intersection => intersection(self.get_static_harbor(coord).unwrap(), f),
                CoordType::Void => void(f),
            }?;
            if (i + 1) % w == 0 {
                writeln!(f)?;
            };
        };
        Ok(())
    }
}
