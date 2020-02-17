mod error;
mod display;
mod player_hand;
mod separated_state;
mod tricell_state;

pub use player_hand::PlayerHand;
pub use separated_state::SeparatedState;
pub use tricell_state::TricellState;
pub use error::Error;
pub type Player = u8;

use crate::utils::{Hex, Harbor, Coord};
use crate::board::layout::Layout;

pub trait StateMaker {
    fn new_empty<'a>(layout: &'a Layout, player_count: u8) -> Box<dyn State + 'a>;
}

pub trait State {

    fn get_layout(&self) -> &Layout;

    fn player_count(&self) -> u8;

    /*** static ***/
    fn set_static_hex(&mut self, coord: Coord, hex: Hex) -> Result<(), Error>;

    fn get_static_hex(&self, coord: Coord) -> Result<Hex, Error>;

    fn set_static_harbor(&mut self, coord: Coord, harbor: Harbor) -> Result<(), Error>;

    fn get_static_harbor(&self, coord: Coord) -> Result<Harbor, Error>;

    /*** dynamic ***/
    fn set_dynamic_path(&mut self, coord: Coord, player: Player) -> Result<(), Error>;

    fn get_dynamic_path(&self, coord: Coord) -> Result<Option<Player>, Error>;

    fn set_dynamic_intersection(&mut self, coord: Coord, player: Player, is_city: bool) -> Result<(), Error>;

    fn get_dynamic_intersection(&self, coord: Coord) -> Result<Option<(Player, bool)>, Error>;
}
