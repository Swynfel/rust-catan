mod environment;
mod py_catan_observation;

use pyo3::prelude::*;

use environment::{SingleEnvironment, MultiEnvironment};
use py_catan_observation::{PyCatanObservation, PyObservationFormat};

#[pymodule]
fn pycatan(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SingleEnvironment>()?;
    m.add_class::<MultiEnvironment>()?;
    m.add_class::<PyObservationFormat>()?;

    Ok(())
}
