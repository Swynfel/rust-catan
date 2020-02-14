mod debug;

use std::ops::{Index, IndexMut};

use crate::board::Coord;
use crate::constants::{Resource, Hex, LandHex, CoordType};
use crate::board::{BoardLayout, TypedCoord};
use crate::error::BoardError;

pub struct State<'a> {
    pub layout: &'a BoardLayout,
    pub board: BoardState<'a>,
    pub players: Vec<PlayerState>,
}

pub struct BoardState<'a> {
    pub hexes: BoardMap<'a, Hex>,
    pub ports: BoardMap<'a, Resource>,
    pub dvp_card: u8,
}

pub struct PlayerState {
    pub resources: [i8; Resource::COUNT],
    pub dvp_carp_vp: u8,
}

pub struct BoardMap<'a, T> {
    map: Vec<Option<T>>,
    layout: &'a BoardLayout,
    coords: CoordType,
}

impl<T: Clone> BoardMap<'_, T> {
    pub fn new(layout: &BoardLayout, coords: CoordType) -> BoardMap<T> {
        BoardMap {
            map: vec![None; layout.size],
            layout,
            coords,
        }
    }
}

impl<T> BoardMap<'_, T> {
    fn get_flat_id_or_fail(&self, coord: Coord) -> Result<usize, BoardError> {
        let flat_id = self.layout.flat_index(coord)?;
        if self.coords != CoordType::Void && self.coords != coord.get_type() {
            return Err(BoardError::WrongCoordType(self.coords, coord.get_type()));
        }
        Ok(flat_id)
    }

    pub fn get(&self, coord: Coord) -> Result<& Option<T>, BoardError> {
        let flat_id = self.get_flat_id_or_fail(coord)?;
        Ok(&self.map[flat_id])
    }

    pub fn get_value(&self, coord: Coord) -> Option<&T> {
        match self.get(coord) {
            Ok(v) => v.as_ref(),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, coord: Coord) -> Result<&mut Option<T>, BoardError> {
        let flat_id = self.get_flat_id_or_fail(coord)?;
        Ok(&mut self.map[flat_id])
    }
}

impl<T> Index<Coord> for BoardMap<'_, T> {
    type Output = Option<T>;

    fn index(&self, coord: Coord) -> &Self::Output {
        self.get(coord).unwrap()
    }
}

impl<T> IndexMut<Coord> for BoardMap<'_, T> {

    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        self.get_mut(coord).unwrap()
    }
}

impl BoardState<'_> {
    pub fn new(layout: &BoardLayout) -> BoardState {
        BoardState {
            hexes: BoardMap::new(layout, CoordType::Hex),
            ports: BoardMap::new(layout, CoordType::Intersection),
            dvp_card: 25,
        }
    }
}

impl State<'_> {
    pub fn new(layout: &BoardLayout, players: usize) -> State {
        State {
            layout,
            board: BoardState::new(layout),
            players: Vec::<PlayerState>::with_capacity(players),
        }
    }
}
