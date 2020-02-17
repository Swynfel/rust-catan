use catan::state::Player;
use once_cell::sync::Lazy;
use termion::color;
use termion::color::AnsiValue;

use catan::utils::{Hex, LandHex, Harbor, Resource};

#[derive(Clone, PartialEq, Debug)]
pub enum DrawType {
    Void,
    Water,
    GenericHarbor,
    Desert,
    Brick,
    Lumber,
    Ore,
    Grain,
    Wool,
}

pub trait ToDrawType where Self: Sized {
    fn to_draw_type(&self) -> DrawType;

    fn to_option_draw_type(optioned: Option<&Self>) -> DrawType {
        match optioned {
            Some(thing) => thing.to_draw_type(),
            None => DrawType::Void,
        }
    }
}

static WHITE: Lazy<AnsiValue> = Lazy::new(|| AnsiValue::rgb(5,5,5));
static GREY: Lazy<AnsiValue> = Lazy::new(|| AnsiValue::rgb(2,2,2));
static BLACK: Lazy<AnsiValue> = Lazy::new(|| AnsiValue::rgb(0,0,0));

impl DrawType {
    pub fn letter(&self) -> char {
        match self {
            DrawType::Void => ' ',
            DrawType::Water => '~',
            DrawType::GenericHarbor => '¤',
            DrawType::Desert => '¤',
            DrawType::Brick => 'B',
            DrawType::Lumber => 'L',
            DrawType::Ore => 'O',
            DrawType::Grain => 'G',
            DrawType::Wool => 'W',
        }
    }

    pub fn fg(&self) -> AnsiValue {
        match self {
            DrawType::Void => *GREY,
            DrawType::Water => *WHITE,
            DrawType::GenericHarbor => *BLACK,
            DrawType::Desert => *GREY,
            DrawType::Brick => *WHITE,
            DrawType::Lumber => *WHITE,
            DrawType::Ore => *WHITE,
            DrawType::Grain => *BLACK,
            DrawType::Wool => *BLACK,
        }
    }

    pub fn bg(&self) -> AnsiValue {
        match self {
            DrawType::Void => AnsiValue::rgb(0,0,0),
            DrawType::Water => AnsiValue::rgb(0,3,5),
            DrawType::GenericHarbor => AnsiValue::rgb(4,4,4),
            DrawType::Desert => AnsiValue::rgb(4,4,3),
            DrawType::Brick => AnsiValue::rgb(4,2,1),
            DrawType::Lumber => AnsiValue::rgb(0,2,1),
            DrawType::Ore => AnsiValue::rgb(2,2,3),
            DrawType::Grain => AnsiValue::rgb(4,4,2),
            DrawType::Wool => AnsiValue::rgb(1,4,2),
        }
    }

    pub fn color(&self) -> String {
        match self {
            DrawType::Void => "".to_string(),
            _ => format!("{}{}", color::Bg(self.bg()), color::Fg(self.fg()))
        }
    }
}

impl ToDrawType for Hex {
    fn to_draw_type(&self) -> DrawType {
        match self {
            Hex::Water => DrawType::Water,
            Hex::Land(landhex) => landhex.to_draw_type(),
        }
     }
}

impl ToDrawType for LandHex {
    fn to_draw_type(&self) -> DrawType {
        match self {
            LandHex::Desert => DrawType::Desert,
            LandHex::Prod(resource, _) => resource.to_draw_type(),
        }
     }
}

impl ToDrawType for Harbor {
    fn to_draw_type(&self) -> DrawType {
        match self {
            Harbor::None => DrawType::Void,
            Harbor::Generic => DrawType::GenericHarbor,
            Harbor::Special(resource) => resource.to_draw_type(),
        }
     }
}

impl ToDrawType for Resource {
    fn to_draw_type(&self) -> DrawType {
        match self {
            Resource::Brick => DrawType::Brick,
            Resource::Lumber => DrawType::Lumber,
            Resource::Ore => DrawType::Ore,
            Resource::Grain => DrawType::Grain,
            Resource::Wool => DrawType::Wool,
        }
     }
}

pub fn player_bg_color(player: Player) -> AnsiValue {
    match player {
        0 => AnsiValue::rgb(5, 1, 1),
        1 => AnsiValue::rgb(1, 1, 5),
        2 => AnsiValue::rgb(5, 5, 5),
        3 => AnsiValue::rgb(4, 4, 1),
        _ => *BLACK,
    }
}

pub fn player_letter(player: Player) -> char {
    match player {
        0 => 'r',
        1 => 'b',
        2 => 'w',
        3 => 'o',
        _ => ' ',
    }
}

pub fn player_color(player: Player) -> String {
    format!("{}{}", color::Bg(player_bg_color(player)), color::Fg(*BLACK))
}
