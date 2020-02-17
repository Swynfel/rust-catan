use super::action::Action;

pub enum Error {
    IncoherentAction(Action),
    IllegalAction(Action),
}
