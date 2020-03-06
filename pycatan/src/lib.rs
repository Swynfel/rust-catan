mod environment;
mod py_catan_observation;

use pyo3::prelude::*;

use environment::Environment;
use py_catan_observation::{PyCatanObservation, PyObservationFormat};

#[pymodule]
fn pycatan(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Environment>()?;
    m.add_class::<PyObservationFormat>()?;

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {

    }
}
