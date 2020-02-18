use std::io::{Write, Error};
use termion::{cursor, color};

use catan::state::State;
use catan::board::{Coord, DetailedCoordType};

const STEP_WIDTH: u16 = 4;
const STEP_WIDTH_BUFFER: u16 = 7;
const STEP_HEIGHT: u16 = 3;
const STEP_HEIGHT_BUFFER: u16 = 4;

pub struct Pattern {
    half_height: u16,
    half_widths: Vec<u16>,
    lines: Vec<&'static str>,
}

impl Pattern {
    pub fn new(lines: Vec<&'static str>) -> Self {
        Pattern {
            half_height: (lines.len() / 2) as u16,
            half_widths: lines.iter().map(|line| line.len() as u16 / 2).collect(),
            lines: lines,
        }
    }

    pub fn display(&self, x: u16, y: u16, f: &mut dyn Write, decorator: &char) ->  Result<(), Error> {
        let ignore = format!("{}", cursor::Right{0:1});
        for (i, (line, half_width)) in self.lines.iter().zip(self.half_widths.iter()).enumerate() {
            let line = line.replace("@", &decorator.to_string()).replace("%", &ignore);
            write!(f, "{}{}", cursor::Goto(x - half_width, y - self.half_height + i as u16), line)?;
        }
        Ok(())
    }
}

fn cursor_position(coord: Coord, half_width: u8, half_height: u8) -> (u16, u16) {
    let x = (coord.x + half_width as i8) as u16;
    let y = (-coord.y + half_height as i8) as u16;
    let x = STEP_WIDTH_BUFFER + x * STEP_WIDTH;
    let y = STEP_HEIGHT_BUFFER + y * STEP_HEIGHT;
    (x, y)
}

pub trait GridDisplayable {
    fn display_hex(&self, x: u16, y: u16, f: &mut dyn Write, coord: Coord, state: &dyn State) ->  Result<(), Error>;

    fn display_path(&self, x: u16, y: u16,  f: &mut dyn Write, coord: Coord, is_i: bool, is_s: bool, state: &dyn State) ->  Result<(), Error>;

    fn display_intersection(&self, x: u16, y: u16,  f: &mut dyn Write, coord: Coord, is_a: bool, state: &dyn State) ->  Result<(), Error>;
}

pub fn grid_display<T : GridDisplayable>(displayable: &T, f: &mut dyn Write, state: &dyn State) -> Result<(),Error> {
    let half_width = state.get_layout().half_width;
    let half_height = state.get_layout().half_height;
    for i in 0..state.get_layout().size {
        let coord = state.get_layout().coord_index(i).unwrap();
        let (x, y) = cursor_position(coord, half_width, half_height);
        match coord.get_detailed_type() {
            DetailedCoordType::OHex => displayable.display_hex(x, y, f, coord, state)?,
            DetailedCoordType::VIntersection => displayable.display_intersection(x, y, f, coord, false, state)?,
            DetailedCoordType::AIntersection => displayable.display_intersection(x, y, f, coord, true, state)?,
            DetailedCoordType::ZPath => displayable.display_path(x, y, f, coord, false, false, state)?,
            DetailedCoordType::SPath => displayable.display_path(x, y, f, coord, false, true, state)?,
            DetailedCoordType::IPath => displayable.display_path(x, y, f, coord, true, false, state)?,
            _ => (),
        };
    };
    write!(f, "{}{}{}",
           color::Fg(color::Reset),
           color::Bg(color::Reset),
           cursor::Goto(1, state.get_layout().height as u16 * STEP_HEIGHT + STEP_HEIGHT_BUFFER))?;
    Ok(())
}
