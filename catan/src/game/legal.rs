use crate::utils::{Coord, CoordType};
use crate::state::State;
use crate::player::Player;
use super::{Action, Error, Phase};

pub fn legal_settlement(coord: Coord, state: &dyn State) -> bool {
    true
}
