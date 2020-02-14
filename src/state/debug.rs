use std::fmt::Display;

use crate::constants::{Hex, LandHex};
use crate::state::BoardState;
use crate::board::TypedCoord;

fn hex(h: Option<&Hex>, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match h {
        Some(h) => match h {
            Hex::Water => water(f),
            Hex::Land(landhex) => match landhex {
                LandHex::Desert => {
                    write!(f, " [ D ] ")
                },
                LandHex::Prod(res, v) => {
                    write!(f, " [{:>2}{}] ", v, res.letter())
                },
            },
        },
        None => water(f),
    }
}

fn path(f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, " x ")
}

fn intersection(f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "<x>")
}

fn void(f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, " ")
}

fn blank(f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "<x>")
}

fn water(f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "  ~~~  ")
}

impl Display for BoardState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let w = self.hexes.layout.width as usize;
        for i in 0..self.hexes.layout.size {
            let coord = self.hexes.layout.coord_index(i).unwrap();
            if i % w == 0 {
                match coord.y.rem_euclid(4) {
                    0 | 1 | 3 => write!(f,"  ")?,
                    _ => (),
                };
            }
            match coord.typed() {
                TypedCoord::Hex(coord) => hex(self.hexes.get_value(coord), f),
                TypedCoord::Path(_) => path(f),
                TypedCoord::Intersection(_) => intersection(f),
                TypedCoord::Void(_) => void(f),
                _ => blank(f),
            }?;
            if (i + 1) % w == 0 {
                writeln!(f)?;
            };
        };
        Ok(())
    }
}
