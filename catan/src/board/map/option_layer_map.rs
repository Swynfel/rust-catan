use super::BoardMap;
use crate::utils::CoordType;
use crate::board::{Coord, Layout, Error};

pub struct OptionLayerMap<T> {
    map: Vec<Option<T>>,
    coords: CoordType,
    layout_half_height: isize,
    layout_half_width: isize,
    layout_width: isize,
}

impl<T: Clone> BoardMap<T> for OptionLayerMap<T> {
    fn get_value(&self, coord: Coord) -> Result<&T, Error> {
        let flat_id = self.get_flat_id_or_fail(coord)?;
        match &self.map[flat_id] {
            Some(value) => Ok(value),
            None => Err(Error::InvalidCoord(coord))
        }
    }

    fn get_mut(&mut self, coord: Coord) -> Result<&mut T, Error> {
        let flat_id = self.get_flat_id_or_fail(coord)?;
        match &mut self.map[flat_id] {
            Some(value) => Ok(value),
            None => Err(Error::InvalidCoord(coord))
        }
    }

    fn set_value(&mut self, coord: Coord, value: T) -> Result<(), Error> {
        let flat_id = self.get_flat_id_or_fail(coord)?;
        self.map[flat_id] = Some(value);
        Ok(())
    }
}

impl<T: Clone> OptionLayerMap<T> {
    pub fn new(layout: &Layout) -> Box<OptionLayerMap<T>> {
        OptionLayerMap::<T>::new_typed(layout, CoordType::Void).boxed()
    }

    pub fn new_typed(layout: &Layout, coords: CoordType) -> OptionLayerMap<T> {
        OptionLayerMap {
            map: vec![None; layout.size],
            coords,
            layout_half_width: layout.half_width as isize,
            layout_half_height: layout.half_height as isize,
            layout_width: layout.width as isize,
        }
    }

    fn get_flat_id_or_fail(&self, coord: Coord) -> Result<usize, Error> {
        let flat_id = Layout::static_flat_index(coord, self.layout_half_width, self.layout_half_height, self.layout_width)?;
        if self.coords != CoordType::Void && self.coords != coord.get_type() {
            return Err(Error::WrongCoordTypeSingle( self.coords, coord.get_type() ));
        }
        Ok(flat_id)
    }
}
