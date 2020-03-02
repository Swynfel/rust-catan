use super::{State, StateTrait};
use crate::board::utils::topology::{RawTopology, TopologyResult};
use crate::board::{Coord, CoordType};

impl<T : StateTrait> RawTopology for T {
    fn neighbours(&self, coord: Coord, center_type: CoordType, neighbour_type: CoordType) -> TopologyResult {
        let results = Coord::TOPOLOGY.neighbours(coord, center_type, neighbour_type)?;
        Ok(results.iter()
            .filter_map(|coord| if self.get_layout().contains_coord(*coord) { Some(*coord) } else { None })
            .collect())
    }
}

impl RawTopology for State {
    fn neighbours(&self, coord: Coord, center_type: CoordType, neighbour_type: CoordType) -> TopologyResult {
        let results = Coord::TOPOLOGY.neighbours(coord, center_type, neighbour_type)?;
        Ok(results.iter()
            .filter_map(|coord| if self.get_layout().contains_coord(*coord) { Some(*coord) } else { None })
            .collect())
    }
}
