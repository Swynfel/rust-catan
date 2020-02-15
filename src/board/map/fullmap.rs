use super::BoardMap;
use crate::utils::CoordType;
use crate::board::{Coord, Layout, Error};

pub struct FullMap<'a, T> {
    map: Vec<Option<T>>,
    layout: &'a Layout,
    coords: CoordType,
}

impl<T: Clone> BoardMap<T> for FullMap<'_, T> {
    fn get_value(&self, coord: &Coord) -> Result<&T, Error> {
        let flat_id = self.get_flat_id_or_fail(coord)?;
        match &self.map[flat_id] {
            Some(value) => Ok(value),
            None => Err(Error::InvalidCoord(*coord))
        }
    }

    fn set_value(&mut self, coord: &Coord, value: T) -> Result<(), Error> {
        let flat_id = self.get_flat_id_or_fail(coord)?;
        self.map[flat_id] = Some(value);
        Ok(())
    }
}

impl<T: Clone> FullMap<'_, T> {
    pub fn new(layout: &Layout) -> Box<FullMap<T>> {
        FullMap {
            map: vec![None; layout.size],
            layout,
            coords: CoordType::Void,
        }.boxed()
    }

    pub fn new_typed(layout: &Layout, coords: CoordType) -> Box<FullMap<T>> {
        FullMap {
            map: vec![None; layout.size],
            layout,
            coords,
        }.boxed()
    }

    fn get_flat_id_or_fail(&self, coord: &Coord) -> Result<usize, Error> {
        let flat_id = self.layout.flat_index(coord)?;
        if self.coords != CoordType::Void && self.coords != coord.get_type() {
            return Err(Error::WrongCoordType(self.coords, coord.get_type()));
        }
        Ok(flat_id)
    }
}
