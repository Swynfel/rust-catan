use ndarray::Array1;
use pyo3::prelude::*;
use numpy::convert::IntoPyArray;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

use catan::game::Game;
use catan::player::{IndexPickerPlayer, PickerPlayerTrait};
use catan::player::Randomy;
use catan::game::{Phase, Action, Error, Notification};
use catan::state::{State, PlayerId};
use catan::board::setup::random_default_setup_existing_state;
use catan::board::layout;
use super::{PyCatanObservation, PyObservationFormat, PythonState};

#[pyclass]
pub struct SingleEnvironment {
    action_sender: Sender<u8>,
    observation_receiver: Receiver<Option<(u8, PyCatanObservation)>>,
    result_receiver: Receiver<(u8,bool)>,
    game_thread: thread::JoinHandle<()>,
}

impl SingleEnvironment {
    fn to_py_tuple(py: Python, observation: Option<(u8, PyCatanObservation)>) -> (PyObject, PyObject, PyObject, PyObject) {
        if let Some((_, observation)) = observation {
            (observation.board.into_pyarray(py).to_object(py), observation.flat.into_pyarray(py).to_object(py), observation.actions.into_pyarray(py).to_object(py), false.into_py(py))
        } else {
            (py.None(), py.None(), py.None(), true.into_py(py))
        }
    }
}

#[pymethods]
impl SingleEnvironment {

    #[staticmethod]
    #[args(format, opponents = 2)]
    fn new(format: &PyObservationFormat, opponents: usize) -> SingleEnvironment {
        let format = *format;
        let (action_sender, action_receiver) = channel();
        let (observation_sender, observation_receiver) = channel();
        let (result_sender, result_receiver) = channel();
        let game_thread = thread::spawn(move || {
            let mut game = Game::new();
            for _ in 0..opponents {
                game.add_player(Box::new(Randomy::new_player()));
            };
            game.add_player(Box::new(IndexPickerPlayer::new(InternalPythonPlayer::new(0, format, action_receiver, observation_sender, result_sender))));
            loop {
                game.setup_and_play();
            }
        });
        SingleEnvironment {
            action_sender,
            observation_receiver,
            result_receiver,
            game_thread,
        }
    }

    fn start(&mut self, py: Python) -> PyResult<(PyObject, PyObject, PyObject, PyObject)> {
        Ok(SingleEnvironment::to_py_tuple(py, self.observation_receiver.recv().expect("Failed to read start observation")))
    }

    fn play(&mut self, py: Python, action: u8) -> PyResult<(PyObject, PyObject, PyObject, PyObject)> {
        self.action_sender.send(action).expect("Failed to send action");
        self.game_thread.thread().unpark();
        Ok(SingleEnvironment::to_py_tuple(py, self.observation_receiver.recv().expect("Failed to read play observation")))
    }

    fn result(&mut self, _py: Python) -> PyResult<(u8,bool)> {
        Ok(self.result_receiver.recv().expect("Failed to read results"))
    }
}


#[pyclass]
pub struct MultiEnvironment {
    players: usize,
    action_senders: Vec<Sender<u8>>,
    observation_receiver: Receiver<Option<(u8, PyCatanObservation)>>,
    result_receivers: Vec<Receiver<(u8,bool)>>,
    game_thread: thread::JoinHandle<()>,
}

impl MultiEnvironment {
    fn to_py_tuple(py: Python, observation: Option<(u8, PyCatanObservation)>) -> (u8, PyObject, PyObject, PyObject, PyObject) {
        if let Some((id, observation)) = observation {
            (id, observation.board.into_pyarray(py).to_object(py), observation.flat.into_pyarray(py).to_object(py), observation.actions.into_pyarray(py).to_object(py), false.into_py(py))
        } else {
            (0, py.None(), py.None(), py.None(), true.into_py(py))
        }
    }
}

#[pymethods]
impl MultiEnvironment {

    #[staticmethod]
    #[args(format, players = 3)]
    fn new(format: &PyObservationFormat, players: usize) -> MultiEnvironment {
        let format = *format;
        let mut action_senders = Vec::new();
        let mut action_receivers = Vec::new();
        let mut result_senders = Vec::new();
        let mut result_receivers = Vec::new();
        for _ in 0..players {
            let (action_sender, action_receiver) = channel();
            let (result_sender, result_receiver) = channel();
            action_senders.push(action_sender);
            action_receivers.push(action_receiver);
            result_senders.push(result_sender);
            result_receivers.push(result_receiver);
        }
        let (observation_sender, observation_receiver) = channel();
        let game_thread = thread::spawn(move || {
            let mut game = Game::new();
            for (id, (action_receiver, result_sender)) in action_receivers.into_iter().zip(result_senders.into_iter()).enumerate() {
                game.add_player(Box::new(IndexPickerPlayer::new(
                    InternalPythonPlayer::new(id as u8, format, action_receiver, observation_sender.clone(), result_sender))));
            };
            let mut rng = SmallRng::from_entropy();
            loop {
                let mut state = PythonState::new(&layout::DEFAULT, players as u8, format);
                random_default_setup_existing_state::<PythonState, SmallRng>(&mut rng, &mut state);
                let mut players_order: Vec<usize> = (0..players).collect();
                players_order.shuffle(&mut rng);
                let mut state: State = Box::new(state);
                game.play(&mut rng, &mut state, players_order);
            }
        });
        MultiEnvironment {
            players,
            action_senders,
            observation_receiver,
            result_receivers,
            game_thread,
        }
    }

    fn start(&mut self, py: Python) -> PyResult<(u8, PyObject, PyObject, PyObject, PyObject)> {
        Ok(MultiEnvironment::to_py_tuple(py, self.observation_receiver.recv().expect("Failed to read start observation")))
    }

    fn play(&mut self, py: Python, player: u8, action: u8) -> PyResult<(u8, PyObject, PyObject, PyObject, PyObject)> {
        self.action_senders[player as usize].send(action).expect("Failed to send action");
        self.game_thread.thread().unpark();
        Ok(MultiEnvironment::to_py_tuple(py, self.observation_receiver.recv().expect("Failed to read play observation")))
    }

    fn result(&mut self, py: Python) -> PyResult<(PyObject, u8)> {
        let mut winner = 0;
        let mut vps = Array1::<u8>::zeros(self.players);
        for player in 0..self.players {
            let result = self.result_receivers[player].recv().expect("Failed to read results");
            vps[player] = result.0;
            if result.1 {
                winner = player;
            }
        }
        Ok((vps.into_pyarray(py).into_py(py), winner as u8))
    }
}

struct InternalPythonPlayer {
    id: u8,
    player: PlayerId,
    observation_format: PyObservationFormat,
    action_receive: Receiver<u8>,
    observation_send: Sender<Option<(u8, PyCatanObservation)>>,
    result_send: Sender<(u8,bool)>,
}

impl InternalPythonPlayer {
    fn new<'a>(id: u8,
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
