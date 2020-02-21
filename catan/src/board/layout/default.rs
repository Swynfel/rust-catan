use once_cell::sync::Lazy;
use std::collections::BTreeSet;

use super::{Layout, Coord, c};
use crate::board::utils::topology::Topology;
use crate::board::Error;

fn paths_from_hexes(hexes: &Vec<Coord>) -> Result<Vec<Coord>, Error> {
    let mut result = BTreeSet::<Coord>::new();
    for hex in hexes.iter() {
        for path in Coord::TOPOLOGY.hex_path_neighbours(*hex)?.into_iter() {
            result.insert(path);
        }
    }
    Ok(result.into_iter().collect())
}

fn intersections_from_hexes(hexes: &Vec<Coord>) -> Result<Vec<Coord>, Error> {
    let mut result = BTreeSet::<Coord>::new();
    for hex in hexes.iter() {
        for intersection in Coord::TOPOLOGY.hex_intersection_neighbours(*hex)?.into_iter() {
            result.insert(intersection);
        }
    }
    Ok(result.into_iter().collect())
}

fn default_layout() -> Layout {
    let hexes = vec![
                  c(-4, -4), c(-4, 0), c(-4, 4),
             c(-2, -6), c(-2,-2), c(-2, 2), c(-2, 6),
        c( 0, -8), c( 0,-4), c( 0, 0), c( 0, 4), c( 0, 8),
             c( 2, -6), c( 2,-2), c( 2, 2), c( 2, 6),
                  c( 4, -4), c( 4, 0), c( 4, 4),
    ];

    let paths = paths_from_hexes(&hexes).expect("Failed getting intersections");

    let intersections = intersections_from_hexes(&hexes).expect("Failed getting intersections");

    Layout::new(2, hexes, paths, intersections)
}

pub static DEFAULT: Lazy<Layout> = Lazy::new(||
    default_layout()
);
