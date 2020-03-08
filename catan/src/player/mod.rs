mod picker_player;
mod randomy;
pub mod relative;

use crate::utils::Empty;
pub use picker_player::{ActionPickerPlayer, IndexPickerPlayer, PickerPlayerTrait};
pub use randomy::Randomy;

use crate::state::{State, PlayerId};
use crate::game::{Action, Notification, Error, Phase};

pub trait CatanPlayer {
    fn new_game(&mut self, position: PlayerId, state: &State);
    fn pick_action(&mut self, phase: &Phase, state: &State) -> Action;
    fn bad_action(&mut self, error: Error);
    fn notify(&mut self, notification: &Notification);
    fn results(&mut self, state: &State, winner: PlayerId);
}

impl<P : Player<Observation = State, Picked = Action, ExtraNew = Empty, ExtraPick = Empty>>
    CatanPlayer for P {
    fn new_game(&mut self, position: PlayerId, state: &State) {
        P::new_game(self, position, state,  Empty{})
    }
    fn pick_action(&mut self, phase: &Phase, state: &State) -> Action {
        P::pick_action(self, phase, state, Empty{})
    }
    fn bad_action(&mut self, error: Error) {
        P::bad_action(self, error)
    }
    fn notify(&mut self, notification: &Notification) {
        P::notify(self, notification)
    }
    fn results(&mut self, state: &State, winner: PlayerId) {
        P::results(self, state, winner)
    }
}

pub trait Player{
    type Observation;
    type Picked;
    type ExtraNew;
    type ExtraPick;

    fn new_game<'f>(&mut self, position: PlayerId, observation: &Self::Observation, extra_new: Self::ExtraNew);
    fn pick_action<'f>(&mut self, phase: &Phase, observation: &Self::Observation, extra_pick: Self::ExtraPick) -> Self::Picked;
    fn bad_action(&mut self, error: Error);
    fn notify(&mut self, notification: &Notification);
    fn results(&mut self, state: &State, winner: PlayerId);
}
