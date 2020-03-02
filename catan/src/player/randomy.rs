use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

use crate::game::{Phase, Action, Error, Notification};
use crate::state::{State, PlayerId};
use super::{CatanPlayer, ActionPickerPlayer, PickerPlayerTrait};

pub struct Randomy {
    rng: SmallRng,
}

impl PickerPlayerTrait for Randomy {
    type ACTIONS = Vec<Action>;
    type PICKED = Action;

    fn new_game(&mut self, _: PlayerId, _: &State, _: &Vec<Action>) {}

    fn pick_action(&mut self, _: &Phase, _: &State, legal_actions: &Vec<Action>) -> Action {
        legal_actions[self.rng.gen_range(0, legal_actions.len())]
    }

    fn bad_action(&mut self, error: Error) {
        println!("{:?}", error);
    }

    fn notify(&mut self, _: &Notification) {}

    fn results(&mut self, _: &State, _:PlayerId) {}
}

impl Randomy {
    fn new() -> Randomy {
        Randomy {
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn new_player() -> impl CatanPlayer {
        ActionPickerPlayer::new(Randomy::new())
    }
}
