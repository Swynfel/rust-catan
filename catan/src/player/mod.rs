mod picker_player;
mod randomy;

pub use picker_player::{ActionPickerPlayer, IndexPickerPlayer, ActionPickerPlayerTrait, IndexPickerPlayerTrait};
pub use randomy::Randomy;

use crate::state::State;
use crate::game::{Action, Notification, Error, Phase};

pub trait Player {
    fn new_game(&mut self, position: u8, state: &dyn State);
    fn pick_action(&mut self, phase: &Phase, state: &dyn State) -> Action;
    fn bad_action(&mut self, error: Error);
    fn notify(&mut self, notification: Notification);
}
