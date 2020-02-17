use super::BoardMap;
use crate::utils::CoordType;
use crate::board::{Coord, Layout, Error};

pub struct LayerMap<T> {
    map: Vec<T>,
    layout_half_height: isize,
    layout_half_width: isize,
    layout_width: isize,
}

impl<T: Copy> BoardMap<T> for LayerMap<T> {
    fn get_value(&self, coord: Coord) -> Result<&T, Error> {
        let flat_id = self.get_flat_id_or_fail(coord)?;
        Ok(&self.map[flat_id])
    }

    fn get_mut(&mut self, coord: Coord) -> Result<&mut T, Error> {
        let flat_id = self.get_flat_id_or_fail(coord)?;
        Ok(&mut self.map[flat_id])
    }

    fn set_value(&mut self, coord: Coord, value: T) -> Result<(), Error> {
        let flat_id = self.get_flat_id_or_fail(coord)?;
        self.map[flat_id] = value;
        Ok(())
    }
}

impl<T: Copy> LayerMap<T> {
    pub fn new(layout: &Layout, default_value: T) -> Box<LayerMap<T>> {
        Box::new(LayerMap {
            map: vec![default_value; layout.size],
            layout_half_width: layout.half_width as isize,
            layout_half_height: layout.half_height as isize,
            layout_width: layout.width as isize,
        })
    }

    fn get_flat_id_or_fail(&self, coord: Coord) -> Result<usize, Error> {
        let flat_id = Layout::static_flat_index(coord, self.layout_half_width, self.layout_half_height, self.layout_width)?;
        Ok(flat_id)
    }
}
