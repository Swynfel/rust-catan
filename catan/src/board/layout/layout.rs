use super::{Coord, Error};

pub struct Layout {
    pub half_width: u8,
    pub half_height: u8,
    pub width: u8,
    pub height: u8,
    pub size: usize,
    pub hexes: Vec<Coord>,
}

impl Layout {
    pub fn new(ray: u8, hexes: Vec<Coord>) -> Layout {
        let half_height = 2*ray + 1;
        let half_width = 2*half_height;
        let width = 2*half_width+1;
        let height = 2*half_height+1;
        Layout {
            half_width,
            half_height,
            width,
            height,
            size: (width as usize)*(height as usize),
            hexes
        }
    }

    pub fn flat_index(&self, coord: Coord) -> Result<usize, Error> {
        let x = coord.x as isize;
        let y = coord.y as isize;
        let half_width = self.half_width as isize;
        let half_height = self.half_height as isize;
        if -half_width <= x && x >= half_width && -half_height <= y && y >= half_height {
            Err(Error::OutOfBoard)
        } else {
            Ok(((half_width + x) + (half_height + y) * self.width as isize) as usize)
        }
    }

    pub fn static_flat_index(coord: Coord, half_width: isize, half_height: isize, width: isize) -> Result<usize, Error> {
        let x = coord.x as isize;
        let y = coord.y as isize;
        if -half_width <= x && x >= half_width && -half_height <= y && y >= half_height {
            Err(Error::OutOfBoard)
        } else {
            Ok(((half_width + x) + (half_height + y) * width) as usize)
        }
    }

    pub fn coord_index(&self, flat: usize) -> Result<Coord, Error> {
        if flat > self.size {
            Err(Error::OutOfBoard)
        } else {
            let x = (flat % self.width as usize) as i8 - self.half_width as i8;
            let y = (flat / self.width as usize) as i8 - self.half_height as i8;
            Ok(Coord::new(x,y))
        }
    }

    pub fn contains_coord(&self, coord: Coord) -> bool {
        let x = coord.x.abs() as u8;
        let y = coord.y.abs() as u8;
        (y <= self.half_height) && (x + y <= self.half_width + 1)
    }
}
