use std::io::{Write, Error};
use termion::{color, cursor, clear, style};

use super::BoardState;
use once_cell::sync::Lazy;
use crate::board::{Coord, DetailedCoordType};
use crate::utils::{Hex, LandHex, Resource, ToDrawType};

const STEP_WIDTH: u16 = 5;
const STEP_WIDTH_BUFFER: u16 = 7;
const STEP_HEIGHT: u16 = 4;
const STEP_HEIGHT_BUFFER: u16 = 4;

struct Pattern {
    half_height: u16,
    half_widths: Vec<u16>,
    lines: Vec<&'static str>,
}

impl Pattern {
    pub fn new(lines: Vec<(u16, &'static str)>) -> Self {
        Pattern {
            half_height: (lines.len() / 2) as u16,
            half_widths: lines.iter().map(|(half_height, _)| *half_height).collect(),
            lines: lines.iter().map(|(_, line)| *line).collect(),
        }
    }
}

static HEX_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new(vec![
         (2, "@@@@@"),
      (5, "@@@     @@@"),
    (7, "@@           @@"),
    (7, "@             @"),
    (7, "@@           @@"),
      (5, "@@@     @@@"),
         (2, "@@@@@"),
]));

static HARBOR_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new(vec![
     (1, "@@@"),
    (2, "@   @"),
     (1, "@@@"),
]));

fn cursor_position(coord: &Coord, board: &BoardState) -> (u16, u16) {
    let x = (coord.x + board.layout.half_width as i8) as u16;
    let y = (-coord.y + board.layout.half_height as i8) as u16;
    let x = STEP_WIDTH_BUFFER + x * STEP_WIDTH;
    let y = STEP_HEIGHT_BUFFER + y * STEP_HEIGHT;
    (x, y)
}

fn display_pattern<W: Write>(x: u16, y: u16, f: &mut W, pattern: &Pattern, decorator: &char) ->  Result<(), Error> {
    for (i, (line, half_width)) in pattern.lines.iter().zip(pattern.half_widths.iter()).enumerate() {
        let line = line.replace("@", &decorator.to_string());
        write!(f, "{}{}", cursor::Goto(x - half_width, y - pattern.half_height + i as u16), line)?;
    }
    Ok(())
}

fn display_hex<W: Write>(coord: &Coord, f: &mut W, board: &BoardState) ->  Result<(), Error>{
    let(x, y) = cursor_position(coord, board);

    let hex = board.hexes.get_value(coord).ok();

    let value = hex.unwrap_or(&Hex::Water).get_num();
    let drawtype = Hex::to_option_draw_type(hex);
    let letter = drawtype.letter();
    let clr = drawtype.color();

    write!(f, "{}", clr)?;

    display_pattern(x, y, f, &HEX_PATTERN, &letter)?;

    if value.is_some() {
        write!(f, "{}{}{:>2}{}", cursor::Goto(x-1, y), style::Bold, value.unwrap(), style::Reset)?;
    }

    write!(f, "{}{}", color::Fg(color::Reset), color::Bg(color::Reset))?;
    Ok(())
}

fn display_intersection<W: Write>(a: bool, coord: &Coord, f: &mut W, board: &BoardState) ->  Result<(), Error>{
    let(x, _y) = cursor_position(coord, board);
    let y;
    if a {
        y = _y - 1;
    } else {
        y = _y + 1;
    }

    if let Some(harbor) = board.harbors.get_value(coord).ok() {
        let drawtype = harbor.to_draw_type();
        let letter = drawtype.letter();
        let clr = drawtype.color();

        write!(f, "{}", clr)?;

        display_pattern(x, y, f, &HARBOR_PATTERN, &letter)?;

        write!(f, "{}{}", color::Fg(color::Reset), color::Bg(color::Reset))?;
    }
    Ok(())
}

/*
fn display_nothing<W: Write>(coord: &Coord, f: &mut W, board: &BoardState) {

}
*/

#[allow(dead_code)]
fn display_debug<W: Write>(coord: &Coord, f: &mut W, board: &BoardState) {
    let (x, y) = cursor_position(coord, board);
    write!(f, "{}{}", cursor::Goto(x, y), coord.get_type()).unwrap();
}

pub fn pretty_terminal_display<W: Write>(f: &mut W, board: &BoardState) -> Result<(),Error> {
    for i in 0..board.layout.size {
        let coord = board.layout.coord_index(i).unwrap();
        match coord.get_detailed_type() {
            DetailedCoordType::OHex => display_hex(&coord, f, board)?,
            DetailedCoordType::VIntersection => display_intersection(false, &coord, f, board)?,
            DetailedCoordType::AIntersection => display_intersection(true, &coord, f, board)?,
            _ => (),
        };
    };
    write!(f, "{}", cursor::Goto(
        board.layout.width as u16 * STEP_WIDTH + STEP_WIDTH_BUFFER,
        board.layout.height as u16 * STEP_HEIGHT + STEP_HEIGHT_BUFFER,
    ))?;
    Ok(())
}
