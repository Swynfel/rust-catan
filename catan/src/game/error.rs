use super::action::Action;
use crate::state::Error as StateError;

#[derive(Copy, Clone, Debug)]
pub enum Error {
    IncoherentAction(Action),
    IllegalAction(Action),
    ImpossibleAction(StateError),
}

impl From<StateError> for Error {
    fn from(state_error: StateError) -> Self {
        Error::ImpossibleAction(state_error)
    }
}
