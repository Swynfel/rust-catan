use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::state::{State, BoardState};
use crate::board::{Coord, BoardLayout};
use crate::constants::{Hex, LandHex, Resource};

pub fn generate_new_state() -> State<'static> {
    let mut state = State::new(&BoardLayout::DEFAULT, 2);
    state.board.randomly_fill();
    state
}

const DEFAULT_LANDTILES: [Option<Resource>; 19] =[
    Some(Resource::Brick), Some(Resource::Brick), Some(Resource::Brick),
    Some(Resource::Lumber), Some(Resource::Lumber), Some(Resource::Lumber), Some(Resource::Lumber),
    Some(Resource::Ore), Some(Resource::Ore), Some(Resource::Ore),
    Some(Resource::Grain), Some(Resource::Grain), Some(Resource::Grain), Some(Resource::Grain),
    Some(Resource::Wool), Some(Resource::Wool), Some(Resource::Wool), Some(Resource::Wool),
    None
];

const DEFAULT_NUM_TOKENS: [u8; 18] = [
    11, 3, 6, 5, 4, 9, 10, 8, 4, 11, 12, 9, 10, 8 ,3, 6, 2, 5
];

const fn c(y:i8, x:i8) -> Coord {
    Coord::new(x,y)
}

pub const DEFAULT_NUM_PATH: [Coord; 19] = [
    c( 0, 0),
    c( 0, 4), c( 2, 2), c( 2,-2), c( 0,-4), c(-2,-2), c(-2, 2),
    c(-2, 6), c( 0, 8), c( 2, 6), c( 4, 4), c( 4, 0), c( 4,-4),
    c( 2,-6), c( 0,-8), c(-2,-6), c(-4,-4), c(-4, 0), c(-4, 4),
];

impl BoardState<'_> {
    pub fn randomly_fill(&mut self) {
        let mut landtiles = DEFAULT_LANDTILES;
        landtiles.shuffle(&mut thread_rng());
        let mut i: usize = 0;
        for (coord, landtile) in DEFAULT_NUM_PATH.iter().zip(landtiles.iter()) {
            self.hexes[*coord] = Some(match landtile {
                Some(res) => {
                    let num_token = DEFAULT_NUM_TOKENS[i];
                    i += 1;
                    Hex::Land(LandHex::Prod(*res, num_token))
                }
                None => Hex::Land(LandHex::Desert)
            })
        }
    }
}
