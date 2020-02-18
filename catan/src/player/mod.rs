use crate::state::State;
use crate::game::{Action, Error};
use crate::game::Phase;

pub trait Player {
    fn new_game(&mut self);
    fn pick_action(&mut self, phase: &Phase, state: &dyn State) -> Action;
    fn bad_action(&mut self, error: Error);
}
