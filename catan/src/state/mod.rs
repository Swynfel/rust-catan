mod display;
mod player_hand;
mod separated_state;
mod tricell_state;
pub mod topology;

pub use player_hand::PlayerHand;
pub use separated_state::SeparatedState;
pub use tricell_state::TricellState;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PlayerId(u8);

use crate::utils::{Hex, Harbor, Coord};
use crate::board::{Layout, Error};

impl PlayerId {
    pub const NONE: PlayerId = PlayerId(std::u8::MAX as u8);
    pub const FIRST: PlayerId = PlayerId(0);

    pub fn to_u8(&self) -> u8 {
        self.0
    }

    pub fn to_usize(&self) -> usize {
        self.to_u8() as usize
    }

    pub fn option(self) -> Option<Self> {
        if self == PlayerId::NONE {
            None
        } else {
            Some(self)
        }
    }
}

impl From<u8> for PlayerId {
    fn from(value: u8) -> Self {
        PlayerId(value)
    }
}

pub trait StateMaker {
    fn new_empty<'a>(layout: &'a Layout, player_count: u8) -> Box<dyn State + 'a>;
}

pub trait State {

    fn get_layout(&self) -> &Layout;

    fn player_count(&self) -> u8;

    fn get_dvp_card_left(&self) -> u8;

    fn get_thief_hex(&self) -> Coord;

    fn set_thief_hex(&mut self, coord: Coord);

    // Player
    fn get_player_hand(&self, player: PlayerId) -> &PlayerHand;

    fn get_player_hand_mut(&mut self, player: PlayerId) -> &mut PlayerHand;

    fn get_player_public_vp(&self, player: PlayerId) -> u8 {
        let mut vp = self.get_player_hand(player).building_vp;
        if let Some((p, _)) = self.get_longest_road() {
            if p == player {
                vp += 2;
            }
        }
        if let Some((p, _)) = self.get_largest_army() {
            if p == player {
                vp += 2;
            }
        }
        vp
    }

    fn get_player_total_vp(&self, player: PlayerId) -> u8 {
        self.get_player_public_vp(player) + self.get_player_hand(player).dvp_cards.victory_point
    }

    fn get_longest_road(&self) -> Option<(PlayerId, u8)>;

    fn get_largest_army(&self) -> Option<(PlayerId, u8)>;

    // Static Board
    fn set_static_hex(&mut self, coord: Coord, hex: Hex) -> Result<(), Error>;

    fn get_static_hex(&self, coord: Coord) -> Result<Hex, Error>;

    fn set_static_harbor(&mut self, coord: Coord, harbor: Harbor) -> Result<(), Error>;

    fn get_static_harbor(&self, coord: Coord) -> Result<Harbor, Error>;

    // Dynamic Board
    fn set_dynamic_path(&mut self, coord: Coord, player: PlayerId) -> Result<(), Error>;

    fn get_dynamic_path(&self, coord: Coord) -> Result<Option<PlayerId>, Error>;

    fn set_dynamic_intersection(&mut self, coord: Coord, player: PlayerId, is_city: bool) -> Result<(), Error>;

    fn get_dynamic_intersection(&self, coord: Coord) -> Result<Option<(PlayerId, bool)>, Error>;
}
