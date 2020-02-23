use catan::utils::{Coord, Resource};

use catan::game::Action;

#[derive(Clone, Debug)]
pub enum ParsingError {
    WrongKeyword(String),
    CouldntParseCoord(String),
    CouldntParseResource(String),
    NotEnoughtParameters,
}

pub fn end_parse_coord(raw: &str) -> Result<Coord, ParsingError> {
    let mut split_raw = raw.split(",");
    let potential_error = || ParsingError::CouldntParseCoord(raw.to_string());
    let x = split_raw.next().ok_or_else(potential_error)?.parse::<i8>().or_else(|_| Err(potential_error()))?;
    let y = split_raw.next().ok_or_else(potential_error)?.parse::<i8>().or_else(|_| Err(potential_error()))?;
    Ok(Coord::new(x,y))
}

pub fn end_parse_resource(raw: &str) -> Result<Resource, ParsingError> {
    match raw.chars().next() {
        Some('B') => Ok(Resource::Brick),
        Some('L') => Ok(Resource::Lumber),
        Some('O') => Ok(Resource::Ore),
        Some('G') => Ok(Resource::Grain),
        Some('W') => Ok(Resource::Wool),
        _ => Err(ParsingError::CouldntParseResource(raw.to_string())),
    }
}

pub fn parse_action(raw: String) -> Result<Action, ParsingError> {
    let raw = raw.replace("\n", "");
    let mut splited = raw.split(" ");
    match splited.next() {
        Some("EndTurn") | Some("E") => {
            Ok(Action::EndTurn)
        }
        Some("Quit") | Some("Q") => {
            Ok(Action::Exit)
        }
        Some("BuildRoad") | Some("Road") | Some("R") => {
            let path = end_parse_coord(splited.next().ok_or(ParsingError::NotEnoughtParameters)?)?;
            Ok(Action::BuildRoad{ path })
        }
        Some("BuildSettlement") | Some("Settlement") | Some("S") => {
            let intersection = end_parse_coord(splited.next().ok_or(ParsingError::NotEnoughtParameters)?)?;
            Ok(Action::BuildSettlement { intersection })
        }
        Some("BuildCity") | Some("City") | Some("C") => {
            let intersection = end_parse_coord(splited.next().ok_or(ParsingError::NotEnoughtParameters)?)?;
            Ok(Action::BuildCity { intersection })
        }
        Some("BuyDevelopmentCard") | Some("DevelopmentCard") | Some("Development") | Some("D") => {
            Ok(Action::BuyDevelopment)
        }
        Some("TradeBank") | Some("Trade") | Some("T") => {
            let given = end_parse_resource(splited.next().ok_or(ParsingError::NotEnoughtParameters)?)?;
            let asked = end_parse_resource(splited.next().ok_or(ParsingError::NotEnoughtParameters)?)?;
            Ok(Action::TradeBank { given, asked })
        }
        Some(other) => {
            Err(ParsingError::WrongKeyword(other.to_string()))
        }
        None => {
            Err(ParsingError::NotEnoughtParameters)
        }
    }
}

pub fn parse_help() -> &'static str {"
Resource: [B]rick [L]umber [O]re [G]rain [W]ool
Coord: <x>,<y>
Action: [E]ndTurn
        Build[R]oad <Coord> / Build[S]ettlement <Coord> / Build[C]ity <Coord>
        Buy[D]evelopmentCard
        [T]radeBank <Resource> <Resource>
        [Q]uit
"}
