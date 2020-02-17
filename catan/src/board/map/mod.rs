mod layer_map;
mod option_layer_map;
mod tricell_map;

pub use layer_map::LayerMap;
pub use option_layer_map::OptionLayerMap;
pub use tricell_map::TricellMap;

use super::{Coord, Error};

pub trait BoardMap<T> {
    fn get_value(&self, coord: Coord) -> Result<&T, Error>;
    fn get_mut(&mut self, coor: Coord) -> Result<&mut T, Error>;
    fn set_value(&mut self, coord: Coord, value: T) -> Result<(), Error>;
    fn boxed(self) -> Box<Self> where Self: std::marker::Sized  {
        Box::new(self)
    }
}
