use ndarray::{Array1, Array3, ArrayD, ArrayViewD, ArrayViewMutD};
use pyo3::prelude::*;
use numpy::{IntoPyArray, PyArrayDyn};

use catan::state::{State, PlayerHand, PlayerId};
use catan::utils::{Hex, LandHex, Harbor, Resource, DevelopmentCard};
use catan::game::{Phase, TurnPhase, DevelopmentPhase};
use catan::player::relative;

use super::{PyObservationFormat, PythonState};

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
pub(crate) struct PyCatanObservation {
    pub actions: Array1<bool>,
    pub board: Array3<i32>,
    pub flat: Array1<i32>,
    pub hidden: Option<Array1<i32>>,
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

    // Fills 27 cells
    pub fn fill_flat_visible(array: &mut Array1::<i32>, index: usize, hand: &PlayerHand, has_longest_road: bool, has_largest_army: bool) {
        for res in 0..Resource::COUNT {
            array[index + res] = hand.resources[res].into();
        }
        array[index + 5] = hand.road_pieces.into();
        array[index + 6] = hand.settlement_pieces.into();
        array[index + 7] = hand.city_pieces.into();
        array[index + 8] = hand.knights.into();
        for d in DevelopmentCard::ALL.iter() {
            array[index + 9 + d.to_usize()] = hand.development_cards[*d].into();
        }
        for d in DevelopmentCard::ALL.iter() {
            array[index + 14 + d.to_usize()] = hand.new_development_cards[*d].into();
        }
        for h in 0..6 {
            array[index + 19 + h] = if hand.harbor[h] { 1 } else { 0 };
        }
        array[index + 25] = if has_longest_road { 1 } else { 0 };
        array[index + 26] = if has_largest_army { 1 } else { 0 };
        //flat[] = state.get_player_total_vp(player).into();
    }

    // Fills 8 cells
    pub fn fill_flat_concealed(array: &mut Array1::<i32>, index: usize, hand: &PlayerHand, has_longest_road: bool, has_largest_army: bool) {
        array[index] = hand.resources.total().into();
        array[index + 1] = hand.road_pieces.into();
        array[index + 2] = hand.settlement_pieces.into();
        array[index + 3] = hand.city_pieces.into();
        array[index + 4] = hand.knights.into();
        array[index + 5] = hand.development_cards.total().into();
        array[index + 6] = if has_longest_road { 1 } else { 0 };
        array[index + 7] = if has_largest_army { 1 } else { 0 };
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
        PyCatanObservation::fill_flat_visible(&mut flat, 0, hand, longest_road == player, largest_army == player);
        // ## Opponents (p-1)*8
        for opp in 1..player_count {
            let player_index = 19+(opp as usize)*8;
            let player = relative::offset_to_player_id(player, opp, player_count);
            let hand = &state.get_player_hand(player);
            PyCatanObservation::fill_flat_concealed(&mut flat, player_index, hand, longest_road == player, largest_army == player);
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


    pub fn generate_hidden(player: PlayerId, state: &State, _phase: &Phase) -> Array1<i32> {
        let player_count = state.player_count();
        let longest_road = match state.get_longest_road() {
            None => PlayerId::NONE,
            Some((player_id, _)) => player_id,
        };
        let largest_army = match state.get_largest_army() {
            None => PlayerId::NONE,
            Some((player_id, _)) => player_id,
        };
        let mut hidden = Array1::<i32>::zeros((player_count as usize - 1)*27);
        // ## Opponents (p-1)*27
        for opp in 1..player_count {
            let player_index = (opp as usize - 1)*27;
            let player = relative::offset_to_player_id(player, opp, player_count);
            let hand = &state.get_player_hand(player);
            PyCatanObservation::fill_flat_concealed(&mut hidden, player_index, hand, longest_road == player, largest_army == player);
        };
        hidden
    }

    pub(crate) fn new_array(format: PyObservationFormat, player: PlayerId, state: &State, phase: &Phase, legal_actions: Array1<bool>) -> PyCatanObservation {
        // # BOARD
        let board = PyCatanObservation::generate_board(format, player, state);

        // # FLAT
        let flat = PyCatanObservation::generate_flat(player, state, phase);

        // # HIDDEN
        let hidden = if format.include_hidden {
            Some(PyCatanObservation::generate_hidden(player, state, phase))
        } else {
            None
        };

        // # RESULT
        PyCatanObservation {
            actions: legal_actions,
            board,
            flat,
            hidden,
        }
    }

    pub(crate) fn new_python_array(format: PyObservationFormat, player: PlayerId, py_state: &PythonState, state: &State, phase: &Phase, legal_actions: Array1<bool>) -> PyCatanObservation {
        // # BOARD
        let board = py_state.boards[player.to_usize()].clone();

        // # FLAT
        let flat = PyCatanObservation::generate_flat(player, state, phase);

        // # HIDDEN
        let hidden = if format.include_hidden {
            Some(PyCatanObservation::generate_hidden(player, state, phase))
        } else {
            None
        };

        // # RESULT
        PyCatanObservation {
            actions: legal_actions,
            board,
            flat,
            hidden,
        }
    }
}
