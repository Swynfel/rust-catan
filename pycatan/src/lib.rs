mod environment;
mod python_state;
mod python_player;
mod py_catan_observation;
mod py_observation_format;

use pyo3::prelude::*;

use environment::{SingleEnvironment, MultiEnvironment};
use python_state::PythonState;
use python_player::PythonPlayer;
use py_catan_observation::PyCatanObservation;
pub use py_observation_format::PyObservationFormat;

#[pymodule]
fn pycatan(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SingleEnvironment>()?;
    m.add_class::<MultiEnvironment>()?;
    m.add_class::<PyObservationFormat>()?;

    Ok(())
}
