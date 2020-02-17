use std::io::{stdout, Stdout, stdin, Write};

use crate::state::State;
use crate::game::Action;
use crate::game::play::Phase;

pub trait Player {
    fn new_game(&mut self);
    fn action_picker(&mut self, phase: &Phase, state: &dyn State) -> Action;
}
