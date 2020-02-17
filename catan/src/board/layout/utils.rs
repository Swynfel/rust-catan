use super::{c, Coord};

fn valid_paths(valid_hexes: [Coord; 19]) -> Vec<Coord> {
    let v = valid_hexes.iter().map(|coord| c(coord.x, coord.y)).collect();
    v
}
