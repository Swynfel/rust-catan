use crate::state::{State, PlayerId};
use crate::game::{Action, Notification, Error, Phase, legal};
use crate::utils::Resource;
use super::CatanPlayer;

pub trait PickerPlayerTrait {
    type ACTIONS;
    type PICKED;

    fn new_game(&mut self, position: PlayerId, state: &State, possible_actions: &Vec<Action>);
    fn pick_action(&mut self, phase: &Phase, state: &State, legal_actions: &Self::ACTIONS) -> Self::PICKED;
    fn bad_action(&mut self, error: Error);
    fn notify(&mut self, notification: &Notification);
    fn results(&mut self, state: &State, winner: PlayerId);
}

pub fn generate_possible_actions(possible_actions: &mut Vec<Action>, player: PlayerId, state: &State) {
    possible_actions.clear();
    let player_count = state.player_count();
    // # BOARD
    // ## Hexes: MoveThief
    for hex in state.get_layout().hexes.iter() {
        for p in 0..state.player_count() {
            let p = p + player.to_u8();
            let p = if p >= player_count { PlayerId::from(p - player_count) } else { PlayerId::from(p) };
            possible_actions.push(Action::MoveThief { hex: *hex, victim: p });
        }
    }
    // ## Paths: BuildRoad
    for path in state.get_layout().paths.iter() {
        possible_actions.push(Action::BuildRoad { path: *path });
    }
    // ## Intersections: BuildSettlement and BuildCity
    for intersection in state.get_layout().intersections.iter() {
        possible_actions.push(Action::BuildSettlement { intersection: *intersection });
        possible_actions.push(Action::BuildCity { intersection: *intersection });
    }
    // # FLAT
    // ## TurnPhase
    possible_actions.push(Action::RollDice);
    possible_actions.push(Action::EndTurn);
    // ## Trade
    for given in Resource::ALL.iter() {
        for asked in Resource::ALL.iter() {
            if given != asked {
                possible_actions.push(Action::TradeBank { given: *given , asked: *asked });
            }
        }
    }
    // ## Development
    possible_actions.push(Action::BuyDevelopment);
    possible_actions.push(Action::DevelopmentKnight);
    possible_actions.push(Action::DevelopmentRoadBuilding);
    possible_actions.push(Action::DevelopmentYearOfPlenty);
    for resource in Resource::ALL.iter() {
        possible_actions.push(Action::ChooseFreeResource { resource: *resource });
    }
    for resource in Resource::ALL.iter() {
        possible_actions.push(Action::DevelopmentMonopole { resource: *resource });
    }
}

pub struct ActionPickerPlayer<T : PickerPlayerTrait<ACTIONS = Vec<Action>, PICKED = Action>> {
    position: PlayerId,
    possible_actions: Vec<Action>,
    action_length: usize,
    player: T,
}

pub struct IndexPickerPlayer<T : PickerPlayerTrait<ACTIONS = Vec<bool>, PICKED = u8>> {
    position: PlayerId,
    possible_actions: Vec<Action>,
    action_length: usize,
    player: T,
}

impl<T : PickerPlayerTrait<ACTIONS = Vec<Action>, PICKED = Action>> ActionPickerPlayer<T> {
    pub fn new(player: T) -> ActionPickerPlayer<T> {
        ActionPickerPlayer {
            position: PlayerId::NONE,
            possible_actions: Vec::new(),
            action_length: 0,
            player,
        }
    }

    fn init_possible_actions(&mut self, state: &State) {
        generate_possible_actions(&mut self.possible_actions, self.position, state);
        self.action_length = self.possible_actions.len();
    }

    fn legal_actions(&mut self, phase: &Phase, state: &State) -> Vec<Action> {
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

impl<T : PickerPlayerTrait<ACTIONS = Vec<bool>, PICKED = u8>> IndexPickerPlayer<T> {
    pub fn new(player: T) -> IndexPickerPlayer<T> {
        IndexPickerPlayer {
            position: PlayerId::NONE,
            possible_actions: Vec::new(),
            action_length: 0,
            player,
        }
    }

    fn init_possible_actions(&mut self, state: &State) {
        generate_possible_actions(&mut self.possible_actions, self.position, state);
        self.action_length = self.possible_actions.len();
    }

    fn legal_actions(&mut self, phase: &Phase, state: &State) -> Vec<bool> {
        let mut legal_actions = Vec::new();
        for action in self.possible_actions.iter() {
            // TODO: More optimized
            // for example, don't check if every road is legal if you can't even afford a road
            legal_actions.push(legal::legal(phase, state, *action).is_ok());
        }
        legal_actions
    }
}

impl<T : PickerPlayerTrait<ACTIONS = Vec<Action>, PICKED = Action>> CatanPlayer for ActionPickerPlayer<T> {
    fn new_game(&mut self, position: PlayerId, state: &State) {
        self.position = position;
        self.init_possible_actions(state);
        self.player.new_game(position, state, &self.possible_actions)
    }

    fn pick_action(&mut self, phase: &Phase, state: &State) -> Action {
        let legal_actions = self.legal_actions(phase, &*state);
        self.player.pick_action(phase, state, &legal_actions)
    }

    fn bad_action(&mut self, error: Error) {
        self.player.bad_action(error)
    }

    fn notify(&mut self, notification: &Notification) {
        self.player.notify(notification)
    }

    fn results(&mut self, state: &State, winner: PlayerId) {
        self.player.results(state, winner)
    }
}

impl<T : PickerPlayerTrait<ACTIONS = Vec<bool>, PICKED = u8>> CatanPlayer for IndexPickerPlayer<T> {
    fn new_game(&mut self, position: PlayerId, state: &State) {
        self.position = position;
        self.init_possible_actions(state);
        self.player.new_game(position, state, &self.possible_actions);
    }

    fn pick_action(&mut self, phase: &Phase, state: &State) -> Action {
        let legal_actions = self.legal_actions(phase, state);
        loop {
            let action = self.player.pick_action(phase, state, &legal_actions) as usize;
            if action < self.possible_actions.len() {
                return self.possible_actions[action as usize];
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

    fn results(&mut self, state: &State, winner: PlayerId) {
        self.player.results(state, winner)
    }
}
