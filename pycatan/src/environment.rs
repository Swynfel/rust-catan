use pyo3::prelude::*;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

use catan::game::Game;
use catan::player::{IndexPickerPlayer, PickerPlayerTrait};
use catan::player::Randomy;
use catan::game::{Phase, Action, Error, Notification};
use catan::state::{State, PlayerId};
use super::PyCatanObservation;

#[pyclass]
pub struct Environment {
    action_send: Sender<u8>,
    observation_receive: Receiver<PyCatanObservation>,
    game_thread: thread::JoinHandle<()>,
}

#[pymethods]
impl Environment {

    #[staticmethod]
    fn new(opponents: usize) -> Environment {
        let (action_send, action_receive) = channel();
        let (observation_send, observation_receive) = channel();
        let game_thread = thread::spawn(move || {
            let mut game = Game::new();
            for _ in 0..opponents {
                game.add_player(Box::new(Randomy::new_player()));
            };
            game.add_player(Box::new(IndexPickerPlayer::new(InternalPythonPlayer::new(action_receive, observation_send))));
            game.play();
        });
        Environment {
            action_send,
            observation_receive,
            game_thread,
        }
    }

    fn start(&mut self) -> PyCatanObservation {
        self.observation_receive.recv().expect("Failed to read observation")
    }

    fn play(&mut self, action: u8) -> PyResult<PyCatanObservation> {
        self.action_send.send(action).expect("Failed to send action");
        self.game_thread.thread().unpark();
        Ok(self.observation_receive.recv().expect("Failed to read observation"))
    }
}

struct InternalPythonPlayer {
    player: PlayerId,
    action_receive: Receiver<u8>,
    observation_send: Sender<PyCatanObservation>,
}

impl InternalPythonPlayer {
    fn new(action_receive: Receiver<u8>, observation_send: Sender<PyCatanObservation>) -> InternalPythonPlayer {
        InternalPythonPlayer {
            player: PlayerId::NONE,
            action_receive,
            observation_send,
        }
    }
}

impl PickerPlayerTrait for InternalPythonPlayer {
    type ACTIONS = Vec<bool>;
    type PICKED = u8;

    fn new_game(&mut self, player: u8, _: &State, _: &Vec<Action>) {
        self.player = PlayerId::from(player);
    }

    fn pick_action(&mut self, _phase: &Phase, state: &State, legal_actions: &Vec<bool>) -> u8 {
        self.observation_send.send(PyCatanObservation::new(self.player, state, legal_actions)).expect("Failed sending observation");
        thread::park();
        self.action_receive.recv().expect("Failed receiving action")
    }

    fn bad_action(&mut self, error: Error) {
        println!("{:?}", error);
    }

    fn notify(&mut self, _: &Notification) {}
}
