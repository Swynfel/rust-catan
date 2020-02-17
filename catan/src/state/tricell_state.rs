
use crate::board::map::TricellMap;
use crate::board::Layout;
use crate::utils::{Empty, Hex, Harbor, Coord};
use super::PlayerHand;
use super::{State, StateMaker, Error, Player};

pub struct TricellState<'a> {
    layout: &'a Layout,
    static_board: Box<TricellMap<Hex,Empty,Harbor>>,
    dynamic_board: Box<TricellMap<Empty,Player,(Player,bool)>>,
    thief: Coord,
    dvp_card: u8,
    players: Vec<PlayerHand>,
}

impl TricellState<'_> {
    const NO_PLAYER: u8 = std::u8::MAX;

    pub fn new(layout: &Layout, players: usize) -> TricellState {
        TricellState {
            layout,
            static_board: TricellMap::new(layout, Hex::Water, Empty::INSTANCE, Harbor::None),
            dynamic_board: TricellMap::new(layout, Empty::INSTANCE, Self::NO_PLAYER, (Self::NO_PLAYER, false)),
            thief: Coord::ZERO,
            dvp_card: 25,
            players: vec![PlayerHand::new();players],
        }
    }
}

impl StateMaker for TricellState<'_> {
    fn new_empty<'a>(layout: &'a Layout, player_count: u8) -> Box<dyn State + 'a> {
        Box::new(TricellState::new(layout, player_count as usize))
    }
}

impl State for TricellState<'_> {

    fn get_layout(&self) -> &Layout {
        self.layout
    }

    fn player_count(&self) -> u8 {
        self.players.len() as u8
    }

    /*** static ***/
    fn set_static_hex(&mut self, coord: Coord, hex: Hex) -> Result<(), Error>{
        Ok(self.static_board.set_hex(coord, hex)?)
    }

    fn get_static_hex(&self, coord: Coord) -> Result<Hex, Error>{
        Ok(self.static_board.get_hex(coord)?)
    }

    fn set_static_harbor(&mut self, coord: Coord, harbor: Harbor) -> Result<(), Error>{
        Ok(self.static_board.set_intersection(coord, harbor)?)
    }

    fn get_static_harbor(&self, coord: Coord) -> Result<Harbor, Error>{
        Ok(self.static_board.get_intersection(coord)?)
    }

    /*** dynamic ***/
    fn set_dynamic_path(&mut self, coord: Coord, player: Player) -> Result<(), Error>{
        Ok(self.dynamic_board.set_path(coord, player)?)
    }

    fn get_dynamic_path(&self, coord: Coord) -> Result<Option<Player>, Error>{
        let player = self.dynamic_board.get_path(coord)?;
        if player < self.player_count() {
            Ok(Some(player))
        } else {
            Ok(None)
        }
    }

    fn set_dynamic_intersection(&mut self, coord: Coord, player: Player, is_city: bool) -> Result<(), Error>{
        Ok(self.dynamic_board.set_intersection(coord, (player, is_city))?)
    }

    fn get_dynamic_intersection(&self, coord: Coord) -> Result<Option<(Player, bool)>, Error>{
        let (player, is_city) = self.dynamic_board.get_intersection(coord)?;
        if player < self.player_count() {
            Ok(Some((player, is_city)))
        } else {
            Ok(None)
        }
    }
}
