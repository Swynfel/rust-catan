use super::State;
use crate::board::utils::topology::{Topology, RawTopology, TopologyResult};
use crate::board::{Coord, CoordType, Error};

impl RawTopology for dyn State {
    fn neighbours(&self, coord: Coord, center_type: CoordType, neighbour_type: CoordType) -> TopologyResult {
        let mut results = Coord::TOPOLOGY.neighbours(coord, center_type, neighbour_type)?;
        Ok(results.iter()
            .filter_map(|coord| if self.get_layout().contains_coord(*coord) { Some(*coord) } else { None })
            .collect())
    }
}

impl Topology for dyn State {
    fn hex_hex_neighbours(&self, coord: Coord) -> TopologyResult {
        self.neighbours(coord, CoordType::Hex, CoordType::Hex)
    }
    fn hex_path_neighbours(&self, coord: Coord) -> TopologyResult {
        self.neighbours(coord, CoordType::Hex, CoordType::Path)
    }
    fn hex_intersection_neighbours(&self, coord: Coord) -> TopologyResult {
        self.neighbours(coord, CoordType::Hex, CoordType::Intersection)
    }

    fn path_hex_neighbours(&self, coord: Coord) -> TopologyResult {
        self.neighbours(coord, CoordType::Path, CoordType::Hex)
    }
    fn path_path_neighbours(&self, coord: Coord) -> TopologyResult {
        self.neighbours(coord, CoordType::Path, CoordType::Path)
    }
    fn path_intersection_neighbours(&self, coord: Coord) -> TopologyResult {
        self.neighbours(coord, CoordType::Path, CoordType::Intersection)
    }

    fn intersection_hex_neighbours(&self, coord: Coord) -> TopologyResult {
        self.neighbours(coord, CoordType::Intersection, CoordType::Hex)
    }
    fn intersection_path_neighbours(&self, coord: Coord) -> TopologyResult {
        self.neighbours(coord, CoordType::Intersection, CoordType::Path)
    }
    fn intersection_intersection_neighbours(&self, coord: Coord) -> TopologyResult {
        self.neighbours(coord, CoordType::Intersection, CoordType::Intersection)
    }
}
