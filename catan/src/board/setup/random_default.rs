use crate::board::utils::topology::Topology;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;

use super::c;
use crate::state::{State, StateMaker};
use crate::board::layout;
use crate::board::utils::{Coord, CoordTransform};
use crate::utils::{Hex, LandHex, Resource, Harbor, DevelopmentCards};

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

const DVP_CARDS: DevelopmentCards =
    DevelopmentCards {
        knight: 14,
        road_building: 2,
        year_of_plenty: 2,
        monopole: 2,
        victory_point: 5,
    };

pub fn random_default_setup_simple<T : StateMaker>(player_count: u8) -> Box<dyn State> {
    random_default_setup::<T, ThreadRng>(&mut thread_rng(), player_count)
}

pub fn random_default_setup<T : StateMaker, R : Rng>(rng: &mut R, player_count: u8) -> Box<dyn State> {
    let mut state = T::new_empty(&layout::DEFAULT, player_count);
    // hexes
    let mut landtiles = LAND_TILES;
    landtiles.shuffle(rng);
    let transform = CoordTransform::random(Coord::ZERO, rng);
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
    porttiles.shuffle(rng);
    let transform = CoordTransform::new(
        Coord::ZERO,
        if rng.gen() {0} else {3},
        false
    );
    let coord_porttile_pairs = PORT_PATHS.iter()
        .map(|&coord| transform.transform(coord))
        .zip(porttiles.iter());
    for (path_coord, &porttile) in coord_porttile_pairs {
        for intersection_coord in state.path_intersection_neighbours(path_coord).expect("Wrong path").iter() {
            state.set_static_harbor(*intersection_coord, porttile)
            .expect("Failed setting harbor");
        }
    };
    // development cards
    *state.get_development_cards_mut() = DVP_CARDS;
    state
}
