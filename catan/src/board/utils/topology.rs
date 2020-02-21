use super::{Coord, CoordType, DetailedCoordType};
use crate::board::Error;

pub type TopologyResult = Result<Vec<Coord>, Error>;

pub trait RawTopology {
    fn neighbours(&self, coord: Coord, center_type: CoordType, neighbour_type: CoordType) -> TopologyResult;
}

pub trait Topology {
    fn hex_hex_neighbours(&self, coord: Coord) -> TopologyResult;
    fn hex_path_neighbours(&self, coord: Coord) -> TopologyResult;
    fn hex_intersection_neighbours(&self, coord: Coord) -> TopologyResult;

    fn path_hex_neighbours(&self, coord: Coord) -> TopologyResult;
    fn path_path_neighbours(&self, coord: Coord) -> TopologyResult;
    fn path_intersection_neighbours(&self, coord: Coord) -> TopologyResult;

    fn intersection_hex_neighbours(&self, coord: Coord) -> TopologyResult;
    fn intersection_path_neighbours(&self, coord: Coord) -> TopologyResult;
    fn intersection_intersection_neighbours<'a>(&self, coord: Coord) -> TopologyResult;
}

pub struct CoordTopology;

fn c(x: i8, y: i8) -> Coord {
    Coord::new(x,y)
}

impl RawTopology for CoordTopology {
    fn neighbours(&self, coord :Coord, center_type: CoordType, neighbour_type: CoordType) -> TopologyResult {
        if coord.get_type() != center_type {
            return Err(Error::WrongCoordType { expected: center_type,  received: coord.get_type() });
        }
        let x = coord.x;
        let y = coord.y;
        Ok(match (coord.get_detailed_type(), neighbour_type) {
            (DetailedCoordType::OHex, CoordType::Hex) => vec![c(x+4,y), c(x+2,y+2), c(x-2,y+2), c(x-4,y), c(x-2,y-2), c(x+2,y-2)],
            (DetailedCoordType::OHex, CoordType::Path) => vec![c(x+2,y), c(x+1,y+1), c(x-1,y+1), c(x-2,y), c(x-1,y-1), c(x+1,y-1)],
            (DetailedCoordType::OHex, CoordType::Intersection) => vec![c(x+2,y+1), c(x,y+1), c(x-2,y+1), c(x-2,y-1), c(x,y-1), c(x+2,y-1)],

            (DetailedCoordType::IPath, CoordType::Hex) => vec![c(x+2,y), c(x-2,y)],
            (DetailedCoordType::SPath, CoordType::Hex) => vec![c(x+1,y+1), c(x-1,y-1)],
            (DetailedCoordType::ZPath, CoordType::Hex) => vec![c(x-1,y+1), c(x+1,y-1)],
            (DetailedCoordType::IPath, CoordType::Path) => vec![c(x+1,y+1), c(x-1,y+1), c(x-1,y-1), c(x+1,y-1)],
            (DetailedCoordType::SPath, CoordType::Path) => vec![c(x+2,y), c(x-1,y+1), c(x-2,y), c(x+1,y-1)],
            (DetailedCoordType::ZPath, CoordType::Path) => vec![c(x+2,y), c(x+1,y+1), c(x-2,y), c(x-1,y-1)],
            (DetailedCoordType::IPath, CoordType::Intersection) => vec![c(x,y+1), c(x,y-1)],
            (DetailedCoordType::SPath, CoordType::Intersection) |
            (DetailedCoordType::ZPath, CoordType::Intersection) => vec![c(x+1,y), c(x-1,y)],

            (DetailedCoordType::AIntersection, CoordType::Hex) => vec![c(x+2,y+1), c(x-2,y+1), c(x,y-1)],
            (DetailedCoordType::VIntersection, CoordType::Hex) => vec![c(x,y+1), c(x-2,y-1), c(x+2,y-1)],
            (DetailedCoordType::AIntersection, CoordType::Path) => vec![c(x+1,y), c(x,y+1), c(x-1,y)],
            (DetailedCoordType::VIntersection, CoordType::Path) => vec![c(x+1,y), c(x-1,y), c(x,y-1)],
            (DetailedCoordType::AIntersection, CoordType::Intersection) => vec![c(x+2,y), c(x,y+2), c(x-2,y)],
            (DetailedCoordType::VIntersection, CoordType::Intersection) => vec![c(x,y+2), c(x-2,y), c(x-2,y)],

            _ => return Err(Error::InvalidNeighbourTypes { center:center_type , neighbours:neighbour_type }),
        })
    }
}

impl<T : RawTopology> Topology for T {
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
