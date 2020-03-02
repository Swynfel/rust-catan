use std::io::{Write, Error};
use once_cell::sync::Lazy;
use termion::{cursor, style, color};

use catan::state::{State, PlayerId, PlayerHand};
use catan::utils::{Resource, Hex, DevelopmentCard};
use catan::board::Coord;

use super::drawtype::{ToDrawType, DrawType, player_color, player_letter};
use super::utils::{GridDisplayable, Pattern};

static HEX_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new(vec![
       "@@@@@@@",
    "@@@       @@@",
    "@           @",
    "@@@       @@@",
       "@@@@@@@",
]));

static INTERSECTION_OUTER_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new(vec![
    "@   @",
    "@   @",
]));

static INTERSECTION_CITY_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new(vec![
    "@@@",
    "@@@",
]));

static INTERSECTION_SETTLEMENT_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new(vec![
     "@",
    " @ ",
]));

static ROAD_I_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new(vec![
    " ",
    "@",
    " ",
]));

static ROAD_S_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new(vec![
    " %%",
     "@",
    "%% ",
]));

static ROAD_Z_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new(vec![
    "%% ",
     "@",
    " %%"
]));

pub(crate) struct PrettyGridDisplay;
impl PrettyGridDisplay {
    pub const INSTANCE: PrettyGridDisplay = PrettyGridDisplay;
}

impl GridDisplayable for PrettyGridDisplay {
    fn display_hex(&self, x: u16, y: u16, f: &mut dyn Write, coord: Coord, state: &State) ->  Result<(), Error>{
        let hex = state.get_static_hex(coord).unwrap_or(Hex::Water);

        let value = hex.get_num();
        let drawtype = hex.to_draw_type();
        let letter = drawtype.letter();
        let clr = drawtype.color();

        write!(f, "{}", clr)?;

        HEX_PATTERN.display(x, y, f, &letter)?;

        if value.is_some() {
            write!(f, "{}{}{:>2}{}", cursor::Goto(x - 1, y), style::Bold, value.unwrap(), style::Reset)?;
        }
        Ok(())
    }

    fn display_path(&self, x: u16, y: u16,  f: &mut dyn Write, coord: Coord, is_i: bool, is_s: bool, state: &State) ->  Result<(), Error> {
        let y = if is_i { y } else { y };

        if let Ok(Some(player)) = state.get_dynamic_path(coord) {
            write!(f, "{}", player_color(player))?;

            if is_i {
                &ROAD_I_PATTERN
            } else {
                if is_s {
                    &ROAD_S_PATTERN
                } else {
                    &ROAD_Z_PATTERN
                }
            }.display(x, y, f, &player_letter(player))?;
        }
        Ok(())
    }


    fn display_intersection(&self, x: u16, y: u16, f: &mut dyn Write, coord: Coord, is_a: bool, state: &State) ->  Result<(), Error> {
        let y = if is_a { y } else { y + 1 };

        if let Some(harbor) = state.get_static_harbor(coord).ok() {
            let drawtype = harbor.to_draw_type();
            if drawtype != DrawType::Void {
                let letter = drawtype.letter();
                let clr = drawtype.color();

                write!(f, "{}", clr)?;

                INTERSECTION_OUTER_PATTERN.display(x, y, f, &letter)?;
            }
        }

        if let Ok(Some((player, is_city))) = state.get_dynamic_intersection(coord) {
            write!(f, "{}", player_color(player))?;

            if is_city {
                &INTERSECTION_CITY_PATTERN
            } else {
                &INTERSECTION_SETTLEMENT_PATTERN
            }.display(x, y, f, &player_letter(player))?;
        }
        Ok(())
    }
}

const FULL_LINE: &str = "                                ";
const WIDTH: u16 = 32;

fn lr_la(player: PlayerId, state: &State) -> String {
    let lr = state.get_longest_road();
    let lr = if let Some((p, _)) = lr {
        if p == player {
            "LR"
        } else {
            "  "
        }
    } else {
        "  "
    };
    let la = state.get_largest_army();
    let la = if let Some((p, _)) = la {
        if p == player {
            "LA"
        } else {
            "  "
        }
    } else {
        "  "
    };
    format!("{} {}", lr, la)
}

pub fn pretty_public_player_hand(f: &mut dyn Write, player: PlayerId, state: &State) ->  Result<(), Error> {
    let hand: &PlayerHand = state.get_player_hand(player);
    let player_color = player_color(player);
    write!(f, "{}{}", player_color, FULL_LINE)?;
    write!(f, "{}{} [{:>2}]", cursor::Left(10), lr_la(player, state), state.get_player_total_vp(player))?;
    write!(f, "{}{} ", cursor::Down(1), cursor::Left(WIDTH))?;
    for resource in Resource::ALL.iter() {
        let resource_draw_type = resource.to_draw_type();
        write!(f, "{resource_color}{resource_letter}{resource_amount:>2}{player_color} ",
            resource_color = resource_draw_type.color(),
            resource_letter = resource_draw_type.letter(),
            resource_amount = hand.resources[*resource],
            player_color = player_color,
        )?;
    }
    write!(f, " ")?;
    let dvp_color = DrawType::GenericHarbor.color();
    for development_card in DevelopmentCard::ALL.iter() {
        write!(f, "{dvp_color}{dvp_amount:>1}{player_color} ",
            dvp_color = dvp_color,
            dvp_amount = hand.development_cards[*development_card],
            player_color = player_color,
        )?;
    }
    write!(f, "{}{}{}{}{}",
        cursor::Down(1),
        cursor::Left(WIDTH),
        FULL_LINE,
        color::Fg(color::Reset),
        color::Bg(color::Reset)
    )?;
    Ok(())
}

pub fn pretty_private_player_hand(f: &mut dyn Write, player: PlayerId, state: &State) ->  Result<(), Error> {
    let hand: &PlayerHand = state.get_player_hand(player);
    let player_color = player_color(player);
    write!(f, "{}{}", player_color, FULL_LINE)?;
    write!(f, "{}{} [{:>2}]", cursor::Left(10), lr_la(player, state), state.get_player_total_vp(player))?;
    write!(f, "{}{} ", cursor::Down(1), cursor::Left(WIDTH))?;
    let generic_color = DrawType::GenericHarbor.color();
    write!(f, "         {generic_color}{resource_amount:>2}{player_color}         ",
        resource_amount = hand.resources.total(),
        generic_color = generic_color,
        player_color = player_color)?;
    write!(f, " ")?;
    write!(f, "    {generic_color}{dvp_amount:>2}{player_color}    ",
        dvp_amount = hand.development_cards.total(),
        generic_color = generic_color,
        player_color = player_color,
    )?;
    write!(f, "{}{}{}{}{}",
        cursor::Down(1),
        cursor::Left(WIDTH),
        FULL_LINE,
        color::Fg(color::Reset),
        color::Bg(color::Reset)
    )?;
    Ok(())
}
