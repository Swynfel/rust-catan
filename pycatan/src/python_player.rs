use ndarray::Array1;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

use catan::state::{State, PlayerId};
use catan::game::{legal, Phase, Action, Error, Notification, TurnPhase, DevelopmentPhase};
use catan::player::CatanPlayer;
use catan::player::generate_possible_actions;
use catan::utils::Resources;

use super::{PythonState, PyCatanObservation, PyObservationFormat};
//generate_possible_actions(&mut self.possible_actions, self.position, state);

pub struct PythonPlayer {
    id: u8,
    position: PlayerId,
    format: PyObservationFormat,
    action_receiver: Receiver<u16>,
    observation_sender: Sender<Option<(u8, PyCatanObservation)>>,
    result_sender: Sender<(u8, bool)>,
    possible_actions: Array1<Action>,
    action_length: usize,
}

impl PythonPlayer {
    pub(crate) fn new<'a>(
        id: u8,
        format: PyObservationFormat,
        action_receiver: Receiver<u16>,
        observation_sender: Sender<Option<(u8, PyCatanObservation)>>,
        result_sender: Sender<(u8,bool)>
    ) -> PythonPlayer {
        PythonPlayer {
            id,
            position: PlayerId::NONE,
            format,
            action_receiver,
            observation_sender,
            result_sender,
            possible_actions: vec![Action::EndTurn;0].into_iter().collect(),
            action_length: 0,
        }
    }

    fn update_legal_actions_slice(&self, legal_actions: &mut Array1<bool>, phase: &Phase, state: &State, from: usize, to: usize) {
        for i in from..to {
            let action = self.possible_actions[i];
            legal_actions[i] = legal::legal(phase, state, action).is_ok();
        }
    }

    fn make_legal_actions(&mut self, phase: &Phase, state: &State) -> Array1<bool> {
        match phase {
            Phase::InitialPlacement { player: _, placing_second: _, placing_road } => self.make_legal_initial_actions(phase, state, *placing_road),
            Phase::Turn { player: _, turn_phase: TurnPhase::Discard(_), development_phase: _ } => self.make_legal_discards(phase, state),
            Phase::Turn { player: _, turn_phase, development_phase } => self.make_legal_turn_actions(phase, state, *turn_phase, *development_phase),
            _ => Array1::default(self.action_length),
        }
    }

    fn make_legal_initial_actions(&mut self, phase: &Phase, state: &State, placing_road: bool) -> Array1<bool> {
        let mut legal_actions = Array1::default(self.action_length);
        let player_count = state.player_count();
        let mut index: usize = state.get_layout().hexes.len() * player_count as usize;
        let path_actions = state.get_layout().paths.len();
        // ## BuildRoad
        if placing_road {
            self.update_legal_actions_slice(&mut legal_actions, phase, state, index, index + path_actions);
        // ## BuildSettlement
        } else {
            index += path_actions;
            for i in 0..state.get_layout().intersections.len() {
                let i = index + 2 * i;
                let action = self.possible_actions[i];
                assert_eq!(legal_actions[i], false);
                legal_actions[i] = legal::legal(phase, state, action).is_ok();
            }
        }
        legal_actions
    }

    fn make_legal_discards(&mut self, phase: &Phase, state: &State) -> Array1<bool> {
        let mut legal_actions = Array1::default(self.action_length);
        self.update_legal_actions_slice(&mut legal_actions, phase, state, self.action_length - 70, self.action_length);
        legal_actions
    }

    fn make_legal_turn_actions(&mut self, phase: &Phase, state: &State, turn_phase: TurnPhase, development_phase: DevelopmentPhase) -> Array1<bool> {
        let mut legal_actions = Array1::default(self.action_length);
        let player_count = state.player_count();
        let hand = state.get_player_hand(self.position);
        let mut index: usize = 0;
        // # BOARD
        // ## Hexes: MoveThief
        let hex_actions = state.get_layout().hexes.len() * player_count as usize;
        if turn_phase == TurnPhase::MoveThief {
            self.update_legal_actions_slice(&mut legal_actions, phase, state, 0, hex_actions);
            return legal_actions;
        } else if development_phase == DevelopmentPhase::KnightActive {
            self.update_legal_actions_slice(&mut legal_actions, phase, state, 0, hex_actions);
        }
        index += hex_actions;
        // ## Paths: BuildRoad
        let path_actions = state.get_layout().paths.len();
        if hand.road_pieces > 0 {
            self.update_legal_actions_slice(&mut legal_actions, phase, state, index, index + path_actions);
        }
        index += path_actions;
        // ## Intersections: BuildSettlement and BuildCity
        let intersection_actions = state.get_layout().intersections.len();
        let can_settlement = hand.settlement_pieces > 0 && hand.resources >= Resources::SETTLEMENT;
        let can_city = hand.city_pieces > 0 && hand.resources >= Resources::CITY;
        if can_settlement {
            for i in 0..intersection_actions {
                let i = index + 2 * i;
                legal_actions[i] = legal::legal(phase, state, self.possible_actions[i]).is_ok();
            }
        }
        if can_city {
            for i in 0..intersection_actions {
                let i = index + 2 * i + 1;
                legal_actions[i] = legal::legal(phase, state, self.possible_actions[i]).is_ok();
            }
        }
        index += 2*intersection_actions;

        // # FLAT
        // ## TurnPhase
        if let Phase::Turn { player: _, turn_phase, development_phase: _ } = phase {
            match turn_phase {
                TurnPhase::PreRoll => legal_actions[index] = true,
                TurnPhase::Free => legal_actions[index+1] = true,
                _ => (),
            }
        }
        index += 2;
        // ## Trade
        self.update_legal_actions_slice(&mut legal_actions, phase, state, index, index + 20);
        index += 20;
        // ## Development
        legal_actions[index] = legal::legal(phase, state, self.possible_actions[index]).is_ok();
        index += 1;
        match development_phase {
            DevelopmentPhase::Ready => {
                let dvp_cards =  hand.development_cards;
                legal_actions[index] = dvp_cards.knight > 0;
                legal_actions[index+1] = dvp_cards.road_building > 0 && hand.road_pieces > 0;
                legal_actions[index+2] = dvp_cards.year_of_plenty > 0;
                if dvp_cards.monopole > 0 {
                    for i in index+8..index+13 {
                        legal_actions[i] = true;
                    }
                }
            }
            DevelopmentPhase::YearOfPlentyActive { two_left: _ } => {
                self.update_legal_actions_slice(&mut legal_actions, phase, state, index + 3, index + 8);
            }
            _ => ()
        }
        legal_actions
    }
}

impl CatanPlayer for PythonPlayer {
    fn new_game(&mut self, position: PlayerId, state: &State) {
        self.position = position;
        if self.action_length == 0 {
            let mut possible_action_vec = Vec::new();
            generate_possible_actions(&mut possible_action_vec, self.position, state);
            self.possible_actions = possible_action_vec.into_iter().collect();
            self.action_length = self.possible_actions.len();
        } else {
            let mut index = 0;
            let player_count = state.player_count();
            for hex in state.get_layout().hexes.iter() {
                for p in 0..state.player_count() {
                    let p = p + position.to_u8();
                    let p = if p >= player_count { PlayerId::from(p - player_count) } else { PlayerId::from(p) };
                    self.possible_actions[index] = Action::MoveThief { hex: *hex, victim: p };
                    index += 1;
                }
            }
        }
    }

    fn pick_action(&mut self, phase: &Phase, state: &State) -> Action {
        let legal_actions = self.make_legal_actions(phase, state);
        self.observation_sender.send(
            Some((
                self.id,
                match state.as_any().downcast_ref::<PythonState>() {
                    Some(python_state) => PyCatanObservation::new_python_array(self.format, self.position, python_state, state, phase, legal_actions),
                    None => PyCatanObservation::new_array(self.format, self.position, state, phase, legal_actions),
                }
            ))
        ).expect("Failed sending observation");
        thread::park();
        self.possible_actions[self.action_receiver.recv().expect("Failed receiving action") as usize]
    }

    fn bad_action(&mut self, error: Error) {
        println!("{:?}", error);
    }

    fn notify(&mut self, _: &Notification) {}

    fn results(&mut self, state: &State, winner: PlayerId) {
        if self.id==0 {
            self.observation_sender.send(None).expect("Failed sending game finished");
        }
        self.result_sender.send((state.get_player_total_vp(self.position), self.position == winner)).expect("Failed sending game results");
    }
}

/*
pub struct InternalPythonPlayer {
    id: u8,
    player: PlayerId,
    observation_format: PyObservationFormat,
    action_receive: Receiver<u8>,
    observation_send: Sender<Option<(u8, PyCatanObservation)>>,
    result_send: Sender<(u8,bool)>,
}

impl InternalPythonPlayer {
    pub(crate) fn new<'a>(id: u8,
        format: PyObservationFormat,
        action_receive: Receiver<u8>,
        observation_send: Sender<Option<(u8, PyCatanObservation)>>,
        result_send: Sender<(u8,bool)>) -> InternalPythonPlayer {
        InternalPythonPlayer {
            id,
            player: PlayerId::NONE,
            observation_format: format,
            action_receive,
            observation_send,
            result_send: result_send,
        }
    }
}

impl PickerPlayerTrait for InternalPythonPlayer {
    type ACTIONS = Vec<bool>;
    type PICKED = u8;

    fn new_game(&mut self, player: PlayerId, _: &State, _: &Vec<Action>) {
        self.player = player;
    }

    fn pick_action(&mut self, phase: &Phase, state: &State, legal_actions: &Vec<bool>) -> u8 {
        self.observation_send.send(
            Some((
                self.id,
                match state.as_any().downcast_ref::<PythonState>() {
                    Some(python_state) => PyCatanObservation::new_python(self.player, python_state, state, phase, legal_actions),
                    None => PyCatanObservation::new(self.observation_format, self.player, state, phase, legal_actions),
                }
            ))
        ).expect("Failed sending observation");
        thread::park();
        self.action_receive.recv().expect("Failed receiving action")
    }

    fn bad_action(&mut self, error: Error) {
        println!("{:?}", error);
    }

    fn notify(&mut self, _notification: &Notification) {}

    fn results(&mut self, state: &State, winner: PlayerId) {
        if self.id==0 {
            self.observation_send.send(None).expect("Failed sending game finished");
        }
        self.result_send.send((state.get_player_total_vp(self.player), self.player == winner)).expect("Failed sending game results");
    }
}
*/
