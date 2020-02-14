use std::fmt;
use crate::constants::CoordType as Type;
use crate::error::BoardError;

pub struct BoardLayout {
    pub half_width: u8,
    pub half_height: u8,
    pub width: u8,
    pub height: u8,
    pub size: usize,
}

#[derive(Copy, Clone)]
pub struct Coord {
    pub x: i8,
    pub y: i8,
}

pub enum TypedCoord {
    Void(Coord),
    Hex(Coord),
    Path(Coord),
    Intersection(Coord),
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coord({},{})", self.x, self.y)
    }
}

impl fmt::Debug for TypedCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypedCoord::Void(coord) => write!(f, "V{:?}", coord),
            TypedCoord::Hex(coord) => write!(f, "H{:?}", coord),
            TypedCoord::Path(coord) => write!(f, "P{:?}", coord),
            TypedCoord::Intersection(coord) => write!(f, "N{:?}", coord),
        }
    }
}

impl BoardLayout {
    pub const fn new(half_width: u8, half_height: u8) -> BoardLayout {
        let width = 2*half_width+1;
        let height = 2*half_height+1;
        BoardLayout {
            half_width,
            half_height,
            width,
            height,
            size: (width as usize)*(height as usize),
        }
    }

    pub const DEFAULT: BoardLayout = BoardLayout::new(10, 5);

    pub fn flat_index(&self, coord: Coord) -> Result<usize, BoardError> {
        let x = coord.x as isize;
        let y = coord.y as isize;
        let half_width = self.half_width as isize;
        let half_height = self.half_height as isize;
        if -half_width <= x && x >= half_width && -half_height <= y && y >= half_height {
            Err(BoardError::OutOfBoard)
        } else {
            Ok(((half_width + x) + (half_height + y) * self.width as isize) as usize)
        }
    }

    pub fn coord_index(&self, flat: usize) -> Result<Coord, BoardError> {
        if flat > self.size {
            Err(BoardError::OutOfBoard)
        } else {
            let x = (flat % self.width as usize) as i8 - self.half_width as i8;
            let y = (flat / self.width as usize) as i8 - self.half_height as i8;
            Ok(Coord::new(x,y))
        }
    }
}

impl Coord {
    pub const fn new(x: i8, y: i8) -> Coord {
        Coord{
            x,
            y,
        }
    }

    pub fn get_type(&self) -> Type {
        let y_r = self.y.rem_euclid(4);
        let y_p = y_r / 2;
        let y_r = y_r % 2;
        let x_r = (self.x + 2 * y_p).rem_euclid(4);
        match (x_r, y_r) {
            (0,0) => Type::Hex,
            (2,0) | (1,1) | (3,1) => Type::Path,
            (0,1) | (2,1) => Type::Intersection,
            _ => Type::Void,
        }
    }

    pub fn typed(self) -> TypedCoord {
        match self.get_type() {
            Type::Void => TypedCoord::Void(self),
            Type::Hex => TypedCoord::Hex(self),
            Type::Path => TypedCoord::Path(self),
            Type::Intersection => TypedCoord::Intersection(self),
        }
    }
}

impl TypedCoord {
    pub fn untyped(self) -> Coord {
        match self {
            TypedCoord::Void(coord) => coord,
            TypedCoord::Hex(coord) => coord,
            TypedCoord::Path(coord) => coord,
            TypedCoord::Intersection(coord) => coord,
        }
    }
}
