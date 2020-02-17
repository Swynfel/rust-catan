use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use super::c;
use crate::state::{State, StateMaker};
use crate::board::layout;
use crate::board::utils::{Coord, CoordTransform};
use crate::utils::{Hex, LandHex, Resource, Harbor};

const LAND_TILES_COUNT: usize = 19;

const LAND_TILES: [Option<Resource>; LAND_TILES_COUNT] = [
    Some(Resource::Brick), Some(Resource::Brick), Some(Resource::Brick),
    Some(Resource::Lumber), Some(Resource::Lumber), Some(Resource::Lumber), Some(Resource::Lumber),
    Some(Resource::Ore), Some(Resource::Ore), Some(Resource::Ore),
    Some(Resource::Grain), Some(Resource::Grain), Some(Resource::Grain), Some(Resource::Grain),
    Some(Resource::Wool), Some(Resource::Wool), Some(Resource::Wool), Some(Resource::Wool),
    None
];

const NUM_TOKENS: [u8; LAND_TILES_COUNT - 1] = [
    11, 3, 6, 5, 4, 9, 10, 8, 4, 11, 12, 9, 10, 8, 3, 6, 2, 5
];

const NUM_TRACK: [Coord; LAND_TILES_COUNT] = [
    c( 0, 0),
    c( 0, 4), c( 2, 2), c( 2,-2), c( 0,-4), c(-2,-2), c(-2, 2),
    c(-2, 6), c( 0, 8), c( 2, 6), c( 4, 4), c( 4, 0), c( 4,-4),
    c( 2,-6), c( 0,-8), c(-2,-6), c(-4,-4), c(-4, 0), c(-4, 4),
];

const PORT_COUNT: usize = 9;

const PORT_TILES: [Harbor; PORT_COUNT] = [
    Harbor::Special(Resource::Brick), Harbor::Special(Resource::Lumber), Harbor::Special(Resource::Ore), Harbor::Special(Resource::Grain), Harbor::Special(Resource::Wool),
    Harbor::Generic, Harbor::Generic, Harbor::Generic, Harbor::Generic
];

const PORT_PATHS: [Coord; PORT_COUNT] = [
    c( 0,10), c( 3, 7), c( 5, 1), c( 5,-5), c( 2,-8),
    c(-2,-8), c(-5,-5), c(-5, 1), c(-3, 7)
];

pub fn random_default_setup<T : State + StateMaker>() -> Box<dyn State> {
    let mut state = T::new_empty(&layout::DEFAULT, 2);
    // BoardState
    let mut rng = thread_rng();
    // hexes
    let mut landtiles = LAND_TILES;
    landtiles.shuffle(&mut rng);
    let transform = CoordTransform::random(Coord::ZERO, &mut rng);
    let coord_landtile_pairs = NUM_TRACK.iter()
        .map(|&coord| transform.transform(coord))
        .zip(landtiles.iter());
    let mut i: usize = 0;
    for (coord, landtile) in coord_landtile_pairs {
        state.set_static_hex(coord, match landtile {
            Some(res) => {
                let num_token = NUM_TOKENS[i];
                i += 1;
                Hex::Land(LandHex::Prod(*res, num_token))
            }
            None => Hex::Land(LandHex::Desert)
        }).expect("Failed setting hexes");
    }
    // ports
    let mut porttiles = PORT_TILES;
    porttiles.shuffle(&mut rng);
    let transform = CoordTransform::new(
        Coord::ZERO,
        if rng.gen() {0} else {3},
        false
    );
    let coord_porttile_pairs = PORT_PATHS.iter()
        .map(|&coord| transform.transform(coord))
        .zip(porttiles.iter());
    for (path_coord, &porttile) in coord_porttile_pairs {
        for intersection_coord in path_coord.path_intersection().expect("Wrong path").iter() {
            state.set_static_harbor(*intersection_coord, porttile)
            .expect("Failed setting harbor");
        }
    };
    state
}

/*
impl BoardState<'_> {
    pub fn randomly_fill(&mut self, setup: &Setup) {
        let mut landtiles = setup.landtiles;
        landtiles.shuffle(&mut thread_rng());
        let mut i: usize = 0;
        for (coord, landtile) in DEFAULT_NUM_PATH.iter().zip(landtiles.iter()) {
            self.hexes.set_value(*coord, match landtile {
                Some(res) => {
                    let num_token = DEFAULT_NUM_TOKENS[i];
                    i += 1;
                    Hex::Land(LandHex::Prod(*res, num_token))
                }
                None => Hex::Land(LandHex::Desert)
            }).expect("Failed setting hexes");
        }
    }
}*/
