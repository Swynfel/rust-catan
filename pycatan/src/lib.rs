mod environment;
mod py_catan_observation;

use pyo3::prelude::*;

use environment::Environment;
use py_catan_observation::PyCatanObservation;

#[pymodule]
fn pycatan(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Environment>()?;

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {

    }
}
