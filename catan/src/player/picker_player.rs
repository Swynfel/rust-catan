use crate::state::State;
use crate::game::{Action, Notification, Error, Phase, legal};
use crate::utils::Resource;
use super::Player;

pub trait ActionPickerPlayerTrait {
    fn new_game(&mut self, position: u8, state: &dyn State, possible_actions: &Vec<Action>);
    fn pick_action(&mut self, phase: &Phase, state: &dyn State, legal_actions: &Vec<Action>) -> Action;
    fn bad_action(&mut self, error: Error);
    fn notify(&mut self, notification: &Notification);
}

pub trait IndexPickerPlayerTrait {
    fn new_game(&mut self, position: u8, state: &dyn State, possible_actions: &Vec<Action>);
    fn pick_action_index(&mut self, phase: &Phase, state: &dyn State, legal_actions: &Vec<bool>) -> u8;
    fn bad_action(&mut self, error: Error);
    fn notify(&mut self, notification: &Notification);
}

pub struct ActionPickerPlayer<T : ActionPickerPlayerTrait> {
    possible_actions: Vec<Action>,
    action_length: usize,
    player: T,
}

pub struct IndexPickerPlayer<T : IndexPickerPlayerTrait> {
    possible_actions: Vec<Action>,
    action_length: usize,
    player: T,
}

impl<T : ActionPickerPlayerTrait> ActionPickerPlayer<T> {
    pub fn new(player: T) -> ActionPickerPlayer<T> {
        ActionPickerPlayer {
            possible_actions: Vec::new(),
            action_length: 0,
            player,
        }
    }

    fn init_possible_actions(&mut self, state: &dyn State) {
        self.possible_actions.clear();
        // EndTurn
        self.possible_actions.push(Action::EndTurn);
        // BuildRoad
        for path in state.get_layout().paths.iter() {
            self.possible_actions.push(Action::BuildRoad { path: *path })
        }
        // BuildSettlement
        for intersection in state.get_layout().intersections.iter() {
            self.possible_actions.push(Action::BuildSettlement { intersection: *intersection })
        }
        // BuildCity
        for intersection in state.get_layout().intersections.iter() {
            self.possible_actions.push(Action::BuildCity { intersection: *intersection })
        }
        // TradeBank
        for given in Resource::ALL.iter() {
            for asked in Resource::ALL.iter() {
                if given != asked {
                    self.possible_actions.push(Action::TradeBank { given: *given , asked: *asked });
                }
            }
        }
        // BuyDvp
        self.possible_actions.push(Action::BuyDvp);
        self.action_length = self.possible_actions.len();
    }

    fn legal_actions(&mut self, phase: &Phase, state: &dyn State) -> Vec<Action> {
        let mut legal_actions = Vec::new();
        for action in self.possible_actions.iter() {
            // TODO: More optimized
            // for example, don't check if every road is legal if you can't even afford a road
            if legal::legal(phase, state, *action).is_ok() {
                legal_actions.push(*action);
            }
        }
        legal_actions
    }
}

impl<T : IndexPickerPlayerTrait> IndexPickerPlayer<T> {
    pub fn new(player: T) -> IndexPickerPlayer<T> {
        IndexPickerPlayer {
            possible_actions: Vec::new(),
            action_length: 0,
            player,
        }
    }

    fn init_possible_actions(&mut self, state: &dyn State) {
        self.possible_actions.clear();
        // EndTurn
        self.possible_actions.push(Action::EndTurn);
        // BuildRoad
        for path in state.get_layout().paths.iter() {
            self.possible_actions.push(Action::BuildRoad { path: *path })
        }
        // BuildSettlement
        for intersection in state.get_layout().intersections.iter() {
            self.possible_actions.push(Action::BuildSettlement { intersection: *intersection })
        }
        // BuildCity
        for intersection in state.get_layout().intersections.iter() {
            self.possible_actions.push(Action::BuildCity { intersection: *intersection })
        }
        // TradeBank
        for given in Resource::ALL.iter() {
            for asked in Resource::ALL.iter() {
                if given != asked {
                    self.possible_actions.push(Action::TradeBank { given: *given , asked: *asked });
                }
            }
        }
        // BuyDvp
        self.possible_actions.push(Action::BuyDvp);
        self.action_length = self.possible_actions.len();
    }

    fn legal_actions(&mut self, phase: &Phase, state: &dyn State) -> Vec<bool> {
        let mut legal_actions = Vec::new();
        for action in self.possible_actions.iter() {
            // TODO: More optimized
            // for example, don't check if every road is legal if you can't even afford a road
            legal_actions.push(legal::legal(phase, state, *action).is_ok());
        }
        legal_actions
    }
}

impl<T: ActionPickerPlayerTrait> Player for ActionPickerPlayer<T> {
    fn new_game(&mut self, position: u8, state: &dyn State) {
        self.init_possible_actions(state);
        self.player.new_game(position, state, &self.possible_actions);
    }
    fn pick_action(&mut self, phase: &Phase, state: &dyn State) -> Action {
        let legal_actions = self.legal_actions(phase, state);
        self.player.pick_action(phase, state, &legal_actions)
    }
    fn bad_action(&mut self, error: Error) {
        self.player.bad_action(error);
    }
    fn notify(&mut self, notification: &Notification) {
        self.player.notify(notification);
    }
}

impl<T: IndexPickerPlayerTrait> Player for IndexPickerPlayer<T> {
    fn new_game(&mut self, position: u8, state: &dyn State) {
        self.init_possible_actions(state);
        self.player.new_game(position, state, &self.possible_actions);
    }
    fn pick_action(&mut self, phase: &Phase, state: &dyn State) -> Action {
        let legal_actions = self.legal_actions(phase, state);
        loop {
            let action = self.player.pick_action_index(phase, state, &legal_actions) as usize;
            if action < self.possible_actions.len() {
                return self.possible_actions[action as usize]
            }
            self.player.bad_action(Error::ActionNotUnderstood)
        }
    }
    fn bad_action(&mut self, error: Error) {
        self.player.bad_action(error);
    }
    fn notify(&mut self, notification: &Notification) {
        self.player.notify(notification);
    }
}
