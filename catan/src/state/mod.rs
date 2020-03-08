mod display;
mod player_hand;
mod tricell_state;
pub mod topology;

pub use player_hand::PlayerHand;
pub use tricell_state::TricellState;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PlayerId(u8);

use std::any::Any;

use crate::utils::{Hex, Harbor, Coord, DevelopmentCards, Resources};
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

impl From<usize> for PlayerId {
    fn from(value: usize) -> Self {
        PlayerId(value as u8)
    }
}

pub trait StateMaker {
    fn new_empty(layout: &'static Layout, player_count: u8) -> State;
}

pub type State = Box<dyn StateTrait>;

pub trait StateTrait {

    fn get_layout(&self) -> &Layout;

    fn player_count(&self) -> u8;

    fn get_development_cards(&self) -> DevelopmentCards;

    fn get_development_cards_mut(&mut self) -> &mut DevelopmentCards;

    fn get_bank_resources(&self) -> Resources;

    fn get_bank_resources_mut(&mut self) -> &mut Resources;

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
        self.get_player_public_vp(player) + self.get_player_hand(player).development_cards.victory_point + self.get_player_hand(player).new_development_cards.victory_point
    }

    fn get_longest_road(&self) -> Option<(PlayerId, u8)>;

    /// Resets and recomputes the longest road of a player
    /// This operation can potentially be expensive since all the possible paths have to be enumerated
    /// It's better to call it only when a player's road has been broken
    /// In most situations the only difference is that a new road piece has been placed. In this case calling [update_longest_road] is more efficient
    fn reset_longest_road(&mut self, player: PlayerId);

    /// Updates a player's longest continous road using a new path
    /// Tries to find the longest road passing through "root_path", and updates the longest continous road of the player if this path is longer
    /// Doesn't look at potential long path not using this road
    /// This function is useful to be called when a new road has been placed, as the new longest road can either be the previous longest road, or a new long road using this new road piece
    /// In more complicated situation, it's better to call [reset_longest_road], but it is more expensive
    fn update_longest_road(&mut self, player: PlayerId, root_path: Coord);

    fn get_largest_army(&self) -> Option<(PlayerId, u8)>;

    fn update_largest_army(&mut self, player: PlayerId);

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

    fn as_any(&self) -> &dyn Any;
}
