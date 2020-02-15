mod fullmap;

pub use fullmap::FullMap;

use super::{Coord, Error};

pub type BoxedMap<'a, T> = Box<dyn BoardMap<T> + 'a>;

pub trait BoardMap<T> {
    fn get_value(&self, coord: &Coord) -> Result<&T, Error>;
    fn set_value(&mut self, coord: &Coord, value: T) -> Result<(), Error>;
    fn boxed(self) -> Box<Self> where Self: std::marker::Sized  {
        Box::new(self)
    }
}
