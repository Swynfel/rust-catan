use ndarray::{Array1, Array3, ArrayD, ArrayViewD, ArrayViewMutD};
use pyo3::prelude::*;
use numpy::{IntoPyArray, PyArrayDyn};

use catan::state::{State, PlayerId};
use catan::utils::{Hex, LandHex, Harbor, Resource};

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

fn jsettlers_u(resource: Resource) -> usize {
    match resource {
        Resource::Brick => 0,
        Resource::Ore => 1,
        Resource::Wool => 2,
        Resource::Grain => 3,
        Resource::Lumber => 4,
    }
}

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
    actions: Array1<bool>,
    board: Array3<i32>,
    flat: Array1<i32>,
}

impl PyCatanObservation {
    pub(crate) fn new(player: PlayerId, state: &State, legal_actions: &Vec<bool>) -> PyCatanObservation {
        // flat
        let mut flat = Array1::<i32>::zeros(11);
        let hand = &state.get_player_hand(player);
        for i in 0..Resource::COUNT {
            flat[i] = hand.resources[jsettlers_resource(i)].into();
        }
        flat[5] = state.get_player_total_vp(player).into();
        flat[6] = state.get_development_cards().total().into();
        flat[7] = hand.road_pieces.into();
        flat[8] = hand.settlement_pieces.into();
        flat[9] = hand.city_pieces.into();
        flat[10] = match state.get_longest_road() {
            None => 0,
            Some((player_id, _)) => {
                if player_id == player {
                    1
                } else {
                    -1
                }
            }
        };
        // board
        let mut board = Array3::<i32>::zeros((21,13,9));
        let layout = state.get_layout();
        for coord in layout.hexes.iter() {
            let hex = state.get_static_hex(*coord).unwrap();
            if let Hex::Land(hex) = hex {
                let x = coord.x as usize + layout.half_width as usize;
                let y = coord.y as usize + layout.half_height as usize;
                match hex {
                    LandHex::Desert => { board[(x, y, 5)] = 1; },
                    LandHex::Prod(res, num) => { board[(x, y, jsettlers_u(res))] = num.into(); },
                }
            }
        };
        for coord in layout.paths.iter() {
            let path = state.get_dynamic_path(*coord).unwrap();
            if let Some(p) = path {
                let x = coord.x as usize + layout.half_width as usize;
                let y = coord.y as usize + layout.half_height as usize;
                board[(x, y, 6)] = if player == p { 1 } else { -1 };
            }
        };
        for coord in layout.intersections.iter() {
            let x = coord.x as usize + layout.half_width as usize;
            let y = coord.y as usize + layout.half_height as usize;
            let harbor = state.get_static_harbor(*coord).unwrap();
            match harbor {
                Harbor::Generic => { board[(x, y, 7)] = 5; }
                Harbor::Special(res) => { board[(x, y, 7)] = jsettlers_u(res) as i32; }
                _ => (),
            }
            let intersection = state.get_dynamic_intersection(*coord).unwrap();
            if let Some((p, is_city)) = intersection {
                let v = if is_city { 2 } else { 1 };
                board[(x, y, 8)] = if player == p { v } else { -v };
            }
        };
        // result
        PyCatanObservation {
            actions: legal_actions.iter().map(|value| *value).collect(),
            board,
            flat,
        }
    }
}
