
use crate::board::map::{OptionLayerMap, BoardMap};
use crate::board::Layout;
use crate::utils::{Hex, Harbor, Coord, CoordType};
use super::PlayerHand;
use super::{State, StateMaker, Error, PlayerId};

pub struct SeparatedState<'a> {
    layout: &'a Layout,
    thief: Coord,
    hexes: OptionLayerMap<Hex>,
    harbors: OptionLayerMap<Harbor>,
    intersections: OptionLayerMap<Option<(u8, bool)>>,
    paths: OptionLayerMap<Option<u8>>,
    dvp_card: u8,
    players: Vec<PlayerHand>,
}

impl SeparatedState<'_> {
    pub fn new(layout: &Layout, players: usize) -> SeparatedState {
        SeparatedState {
            layout,
            thief: Coord::ZERO,
            hexes: OptionLayerMap::new_typed(layout, CoordType::Hex),
            harbors: OptionLayerMap::new_typed(layout, CoordType::Intersection),
            intersections: OptionLayerMap::new_typed(layout, CoordType::Intersection),
            paths: OptionLayerMap::new_typed(layout, CoordType::Path),
            dvp_card: 25,
            players: vec![PlayerHand::new();players],
        }
    }
}

impl StateMaker for SeparatedState<'_> {
    fn new_empty<'a>(layout: &'a Layout, player_count: u8) -> Box<dyn State + 'a> {
        Box::new(SeparatedState::new(layout, player_count as usize))
    }
}

impl State for SeparatedState<'_> {

    fn get_layout(&self) -> &Layout {
        self.layout
    }

    fn player_count(&self) -> u8 {
        self.players.len() as u8
    }

    fn get_player_hand(&self, player: PlayerId) -> PlayerHand {
        self.players[player as usize]
    }

    /*** static ***/
    fn set_static_hex(&mut self, coord: Coord, hex: Hex) -> Result<(), Error>{
        Ok(self.hexes.set_value(coord, hex)?)
    }

    fn get_static_hex(&self, coord: Coord) -> Result<Hex, Error>{
        Ok(*self.hexes.get_value(coord)?)
    }

    fn set_static_harbor(&mut self, coord: Coord, harbor: Harbor) -> Result<(), Error>{
        Ok(self.harbors.set_value(coord, harbor)?)
    }

    fn get_static_harbor(&self, coord: Coord) -> Result<Harbor, Error>{
        Ok(*self.harbors.get_value(coord)?)
    }

    /*** dynamic ***/
    fn set_dynamic_path(&mut self, coord: Coord, player: PlayerId) -> Result<(), Error>{
        Ok(self.paths.set_value(coord, Some(player))?)
    }

    fn get_dynamic_path(&self, coord: Coord) -> Result<Option<PlayerId>, Error>{
        Ok(*self.paths.get_value(coord)?)
    }

    fn set_dynamic_intersection(&mut self, coord: Coord, player: PlayerId, is_city: bool) -> Result<(), Error>{
        Ok(self.intersections.set_value(coord, Some((player, is_city)))?)
    }

    fn get_dynamic_intersection(&self, coord: Coord) -> Result<Option<(PlayerId, bool)>, Error>{
        Ok(*self.intersections.get_value(coord)?)
    }
}
