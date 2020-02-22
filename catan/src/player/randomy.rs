use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

use crate::game::{Phase, Action, Error, Notification};
use crate::state::State;
use super::{Player, ActionPickerPlayer, ActionPickerPlayerTrait};

pub struct Randomy {
    rng: SmallRng,
}

impl ActionPickerPlayerTrait for Randomy {
    fn new_game(&mut self, _: u8, _: &dyn State, _: &Vec<Action>) {}

    fn pick_action(&mut self, _: &Phase, _: &dyn State, legal_actions: &Vec<Action>) -> Action {
        legal_actions[self.rng.gen_range(0, legal_actions.len())]
    }

    fn bad_action(&mut self, error: Error) {
        println!("{:?}", error);
    }

    fn notify(&mut self, _: &Notification) {}
}

impl Randomy {
    fn new() -> Randomy {
        Randomy {
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn new_player() -> impl Player {
        ActionPickerPlayer::new(Randomy::new())
    }
}
