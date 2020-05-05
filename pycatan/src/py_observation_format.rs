use pyo3::prelude::*;
use catan::utils::{Coord};

#[pyclass]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PyObservationFormat {
    pub half_width: usize,
    pub half_height: usize,
    pub width: usize,
    pub height: usize,
    pub use_python_state: bool,
    pub include_hidden: bool,
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
    #[args(half_width = 10, half_height = 5, use_python_state = false, include_hidden = false)]
    pub fn new(half_width: usize, half_height: usize, use_python_state: bool, include_hidden: bool) -> Self {
        PyObservationFormat {
            half_width,
            half_height,
            width: 2*half_width+1,
            height: 2*half_height+1,
            use_python_state,
            include_hidden,
        }
    }
}
