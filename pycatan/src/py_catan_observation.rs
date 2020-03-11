use ndarray::{Array1, Array3, ArrayD, ArrayViewD, ArrayViewMutD};
use pyo3::prelude::*;
use numpy::{IntoPyArray, PyArrayDyn};

use catan::state::{State, PlayerId};
use catan::utils::{Hex, LandHex, Harbor, Resource, Coord, DevelopmentCard};
use catan::game::{Phase, TurnPhase, DevelopmentPhase};
use catan::player::relative;

use super::PythonState;

#[pymodule]
fn rust_ext(_py: Python, m: &PyModule) -> PyResult<()> {
    // immutable example
    fn axpy(a: f64, x: ArrayViewD<f64>, y: ArrayViewD<f64>) -> ArrayD<f64> {
        a * &x + &y
    }

    // mutable example (no return)
    fn mult(a: f64, mut x: ArrayViewMutD<f64>) {
        x *= a;
    }

    // wrapper of `axpy`
    #[pyfn(m, "axpy")]
    fn axpy_py(
        py: Python,
        a: f64,
        x: &PyArrayDyn<f64>,
        y: &PyArrayDyn<f64>,
    ) -> Py<PyArrayDyn<f64>> {
        let x = x.as_array();
        let y = y.as_array();
        axpy(a, x, y).into_pyarray(py).to_owned()
    }

    // wrapper of `mult`
    #[pyfn(m, "mult")]
    fn mult_py(_py: Python, a: f64, x: &PyArrayDyn<f64>) -> PyResult<()> {
        let x = x.as_array_mut();
        mult(a, x);
        Ok(())
    }

    Ok(())
}

#[allow(dead_code)]
fn jsettlers_u(resource: Resource) -> usize {
    match resource {
        Resource::Brick => 0,
        Resource::Ore => 1,
        Resource::Wool => 2,
        Resource::Grain => 3,
        Resource::Lumber => 4,
    }
}

#[allow(dead_code)]
fn jsettlers_resource(value: usize) -> Resource {
    match value {
        0 => Resource::Brick,
        1 => Resource::Ore,
        2 => Resource::Wool,
        3 => Resource::Grain,
        4 => Resource::Lumber,
        _ => panic!("Bad value for resource"),
    }
}

#[pyclass]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PyObservationFormat {
    pub half_width: usize,
    pub half_height: usize,
    pub width: usize,
    pub height: usize,
    pub use_python_state: bool,
}

impl PyObservationFormat {
    pub fn map(&self, coord: Coord) -> (usize, usize) {
        let x = (coord.x + self.half_width as i8) as usize;
        let y = (coord.y + self.half_height as i8) as usize;
        (x,y)
    }
}

#[pymethods]
impl PyObservationFormat {

    #[new]
    #[staticmethod]
    #[args(half_width = 10, half_height = 5, use_python_state = false)]
    pub fn new(half_width: usize, half_height: usize, use_python_state: bool) -> Self {
        PyObservationFormat {
            half_width,
            half_height,
            width: 2*half_width+1,
            height: 2*half_height+1,
            use_python_state,
        }
    }
}

#[pyclass]
pub(crate) struct PyCatanObservation {
    pub actions: Array1<bool>,
    pub board: Array3<i32>,
    pub flat: Array1<i32>,
}

impl PyCatanObservation {
    pub fn generate_board(format: PyObservationFormat, player: PlayerId, state: &State) -> Array3<i32> {
        let player_count = state.player_count();
        let mut board = Array3::<i32>::zeros((format.width,format.height, 13 + 2 * player_count as usize));
        let layout = state.get_layout();
        // ## Hexes [0,7[
        for coord in layout.hexes.iter() {
            let hex = state.get_static_hex(*coord).unwrap();
            if let Hex::Land(hex) = hex {
                let (x,y) = format.map(*coord);
                match hex {
                    LandHex::Desert => { board[(x, y, 5)] = 1; },
                    LandHex::Prod(res, num) => { board[(x, y, res.to_usize())] = num.into(); },
                }
                if *coord == state.get_thief_hex() {
                    board[(x, y, 6)] = 1;
                }
            }
        };
        // ## Paths [7,7+player_count[
        let c = 7;
        for coord in layout.paths.iter() {
            let path = state.get_dynamic_path(*coord).unwrap();
            if let Some(p) = path {
                let p = relative::player_id_to_relative(player, p, player_count);
                let (x,y) = format.map(*coord);
                board[(x, y, c + p.to_usize())] = 1;
            }
        };
        let c_harbor = 7 + player_count as usize;
        let c_buildings = c_harbor + 6;
        // ## Intersections [7+player_count, 13+2×player_count[
        for coord in layout.intersections.iter() {
            let (x,y) = format.map(*coord);
            let harbor = state.get_static_harbor(*coord).unwrap();
            match harbor {
                Harbor::Generic => { board[(x, y, c_harbor + 5)] = 1; }
                Harbor::Special(res) => { board[(x, y, c_harbor + res.to_usize())] = 1; }
                _ => (),
            }
            let intersection = state.get_dynamic_intersection(*coord).unwrap();
            if let Some((p, is_city)) = intersection {
                let p = relative::player_id_to_relative(player, p, player_count);
                board[(x, y, c_buildings + p.to_usize())] = if is_city { 2 } else { 1 };
            }
        };
        board
    }

    pub fn generate_flat(player: PlayerId, state: &State, phase: &Phase) -> Array1<i32> {
        let player_count = state.player_count();
        let mut flat = Array1::<i32>::zeros(29+(player_count as usize)*8);
        let longest_road = match state.get_longest_road() {
            None => PlayerId::NONE,
            Some((player_id, _)) => player_id,
        };
        let largest_army = match state.get_largest_army() {
            None => PlayerId::NONE,
            Some((player_id, _)) => player_id,
        };
        // ## Player 27
        let hand = &state.get_player_hand(player);
        for res in 0..Resource::COUNT {
            flat[res] = hand.resources[res].into();
        }
        flat[5] = hand.road_pieces.into();
        flat[6] = hand.settlement_pieces.into();
        flat[7] = hand.city_pieces.into();
        flat[8] = hand.knights.into();
        for d in DevelopmentCard::ALL.iter() {
            flat[9+d.to_usize()] = hand.development_cards[*d].into();
        }
        for d in DevelopmentCard::ALL.iter() {
            flat[14+d.to_usize()] = hand.new_development_cards[*d].into();
        }
        for h in 0..6 {
            flat[19+h] = if hand.harbor[h] { 1 } else { 0 };
        }
        flat[25] = if longest_road == player { 1 } else { 0 };
        flat[26] = if largest_army == player { 1 } else { 0 };
        //flat[] = state.get_player_total_vp(player).into();
        // ## Opponents (p-1)*8
        for opp in 1..player_count {
            let c_player = 19+(opp as usize)*8;
            let player = relative::offset_to_player_id(player, opp, player_count);
            let hand = &state.get_player_hand(player);
            flat[c_player] = hand.resources.total().into();
            flat[c_player+1] = hand.road_pieces.into();
            flat[c_player+2] = hand.settlement_pieces.into();
            flat[c_player+3] = hand.city_pieces.into();
            flat[c_player+4] = hand.knights.into();
            flat[c_player+5] = hand.development_cards.total().into();
            flat[c_player+6] = if longest_road == player { 1 } else { 0 };
            flat[c_player+7] = if largest_army == player { 1 } else { 0 };
        }
        // ## State 6
        let c_state = 19+(player_count as usize)*8;
        let bank_resources = state.get_bank_resources();
        for res in 0..Resource::COUNT {
            flat[c_state + res] = bank_resources[res].into();
        }
        flat[c_state+5] = state.get_development_cards().total().into();
        // ## Phase 4
        let c_phase = c_state + 6;
        if let Phase::Turn { player: _, turn_phase, development_phase } = phase {
            flat[c_phase] = if let TurnPhase::PreRoll = turn_phase { 1 } else { 0 };
            flat[c_phase+1] = if let DevelopmentPhase::Ready = development_phase { 1 } else { 0 };
            flat[c_phase+2] = if let DevelopmentPhase::RoadBuildingActive { two_left } = development_phase { if *two_left { 2 } else { 1 } } else { 0 };
            flat[c_phase+3] = if let DevelopmentPhase::YearOfPlentyActive { two_left } = development_phase { if *two_left { 2 } else { 1 } } else { 0 };
        }
        flat
    }

    #[allow(dead_code)]
    pub(crate) fn new_vec(format: PyObservationFormat, player: PlayerId, state: &State, phase: &Phase, legal_actions: &Vec<bool>) -> PyCatanObservation {
        // # BOARD
        let board = PyCatanObservation::generate_board(format, player, state);

        // # FLAT
        let flat = PyCatanObservation::generate_flat(player, state, phase);

        // # RESULT
        PyCatanObservation {
            actions: legal_actions.iter().map(|value| *value).collect(),
            board,
            flat,
        }
    }

    pub(crate) fn new_array(format: PyObservationFormat, player: PlayerId, state: &State, phase: &Phase, legal_actions: Array1<bool>) -> PyCatanObservation {
        // # BOARD
        let board = PyCatanObservation::generate_board(format, player, state);

        // # FLAT
        let flat = PyCatanObservation::generate_flat(player, state, phase);

        // # RESULT
        PyCatanObservation {
            actions: legal_actions,
            board,
            flat,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_python_vec(player: PlayerId, py_state: &PythonState, state: &State, phase: &Phase, legal_actions: &Vec<bool>) -> PyCatanObservation {
        // # BOARD
        let board = py_state.boards[player.to_usize()].clone();

        // # FLAT
        let flat = PyCatanObservation::generate_flat(player, state, phase);

        // # RESULT
        PyCatanObservation {
            actions: legal_actions.iter().map(|value| *value).collect(),
            board,
            flat,
        }
    }

    pub(crate) fn new_python_array(player: PlayerId, py_state: &PythonState, state: &State, phase: &Phase, legal_actions: Array1<bool>) -> PyCatanObservation {
        // # BOARD
        let board = py_state.boards[player.to_usize()].clone();

        // # FLAT
        let flat = PyCatanObservation::generate_flat(player, state, phase);

        // # RESULT
        PyCatanObservation {
            actions: legal_actions,
            board,
            flat,
        }
    }
}
