use super::BoardMap;
use crate::utils::CoordType;
use crate::board::{Coord, Layout, Error};

#[derive(Copy, Clone)]
union Tricell<H: Copy, P: Copy, I: Copy> {
    hex: H,
    path: P,
    intersection: I,
}

pub struct TricellMap<H: Copy, P: Copy, I: Copy> {
    map: Vec<Option<Tricell<H,P,I>>>,
    defaults: (H,P,I),
    layout_half_height: isize,
    layout_half_width: isize,
    layout_width: isize,
}

impl<H: Copy, P: Copy, I: Copy> TricellMap<H,P,I> {
    pub fn new(layout: &Layout, default_hex: H, default_path: P, default_intersection: I) -> Box<TricellMap<H,P,I>> {
        Box::new(TricellMap {
            map: vec![None; layout.size],
            defaults: (default_hex, default_path, default_intersection),
            layout_half_width: layout.half_width as isize,
            layout_half_height: layout.half_height as isize,
            layout_width: layout.width as isize,
        })
    }

    fn assert_type(coord: Coord, coord_type: CoordType) -> Result<(),Error> {
        if coord_type != coord.get_type() {
            Err(Error::WrongCoordType{ expected: coord_type, received: coord.get_type() })
        } else {
            Ok(())
        }
    }

    fn get_id_or_fail(&self, coord: Coord) -> Result<usize, Error> {
        let flat_id = Layout::static_flat_index(coord, self.layout_half_width, self.layout_half_height, self.layout_width)?;
        Ok(flat_id)
    }

    pub fn get_hex(&self, coord: Coord) -> Result<H, Error> {
        Self::assert_type(coord, CoordType::Hex)?;
        let flat_id = self.get_id_or_fail(coord)?;
        Ok(match self.map[flat_id] {
            None => self.defaults.0,
            Some(tri_cell) => unsafe { tri_cell.hex },
        })
    }

    pub fn set_hex(&mut self, coord: Coord, value: H) -> Result<(), Error> {
        Self::assert_type(coord, CoordType::Hex)?;
        let flat_id = self.get_id_or_fail(coord)?;
        self.map[flat_id] = Some(Tricell { hex: value });
        Ok(())
    }

    pub fn get_path(&self, coord: Coord) -> Result<P, Error> {
        Self::assert_type(coord, CoordType::Path)?;
        let flat_id = self.get_id_or_fail(coord)?;
        Ok(match self.map[flat_id] {
            None => self.defaults.1,
            Some(tri_cell) => unsafe { tri_cell.path },
        })
    }

    pub fn set_path(&mut self, coord: Coord, value: P) -> Result<(), Error> {
        Self::assert_type(coord, CoordType::Path)?;
        let flat_id = self.get_id_or_fail(coord)?;
        self.map[flat_id] = Some(Tricell { path: value });
        Ok(())
    }

    pub fn get_intersection(&self, coord: Coord) -> Result<I, Error> {
        Self::assert_type(coord, CoordType::Intersection)?;
        let flat_id = self.get_id_or_fail(coord)?;
        Ok(match self.map[flat_id] {
            None => self.defaults.2,
            Some(tri_cell) => unsafe { tri_cell.intersection },
        })
    }

    pub fn set_intersection(&mut self, coord: Coord, value: I) -> Result<(), Error> {
        Self::assert_type(coord, CoordType::Intersection)?;
        let flat_id = self.get_id_or_fail(coord)?;
        self.map[flat_id] = Some(Tricell { intersection: value });
        Ok(())
    }
}
