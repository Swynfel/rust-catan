use crate::utils::{Coord, CoordType};
use crate::state::{State, PlayerId};
use crate::board::utils::topology::Topology;
use crate::board::Error;

/// Is the intersection free for a settlement
///
/// Returns true if there is no settlement at the coord or around it
pub fn available_settlement_position(coord: Coord, state: &dyn State) -> Result<bool, Error> {
    let neighbours = state.intersection_intersection_neighbours(coord)?;
    for neighbour_intersection in neighbours {
        if state.get_dynamic_intersection(neighbour_intersection)?.is_some() {
            return Ok(false);
        }
    }
    return Ok(state.get_dynamic_intersection(coord)?.is_none());
}

/// Is this position allowed for a inital placement road
///
/// Returns true if the path or intersection coord is next to a road owned by the player
pub fn allowed_initial_road_placement(coord: Coord, player: PlayerId, state: &dyn State) -> Result<bool, Error> {
    let neighbours = state.path_intersection_neighbours(coord)?;
    let mut neighbour_settlement = None;
    for neighbour in neighbours {
        if let Some((p, _)) = state.get_dynamic_intersection(neighbour)? {
            if player == p {
                neighbour_settlement = Some(neighbour);
            }
        }
    }
    if let Some(neighbour_settlement) = neighbour_settlement {
        let connected = connected_position(neighbour_settlement, player, state)?;
        // If the settlement is already connected it means we are putting the player is placing the road next to the wrong selltement
        Ok(!connected)
    } else {
        Ok(false)
    }
}

/// Is the path or intersection connected to a piece owned by the player
///
/// Returns true if the path or intersection coord is next to a road owned by the player
pub fn connected_position(coord: Coord, player: PlayerId, state: &dyn State) -> Result<bool, Error> {
    let neighbours = match coord.get_type() {
        CoordType::Path => state.path_path_neighbours(coord)?,
        CoordType::Intersection => state.intersection_path_neighbours(coord)?,
        t => return Err(Error::WrongCoordType { expected:[false, false, true, true], received:t }),
    };
    for neighbour in neighbours {
        if let Some(p) = state.get_dynamic_path(neighbour)? {
            if player == p {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
