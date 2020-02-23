use crate::board::map::TricellMap;
use crate::board::{Layout, Error};
use crate::utils::{Empty, Hex, Harbor, Coord, DevelopmentCards};
use super::PlayerHand;
use super::{State, StateMaker, PlayerId};

pub struct TricellState<'a> {
    layout: &'a Layout,
    static_board: Box<TricellMap<Hex,Empty,Harbor>>,
    dynamic_board: Box<TricellMap<Empty,PlayerId,(PlayerId,bool)>>,
    thief: Coord,
    development_card: DevelopmentCards,
    longest_road: PlayerId,
    largest_army: PlayerId,
    players: Vec<PlayerHand>,
}

impl TricellState<'_> {
    pub fn new(layout: &Layout, players: usize) -> TricellState {
        TricellState {
            layout,
            static_board: TricellMap::new(layout, Hex::Water, Empty::INSTANCE, Harbor::None),
            dynamic_board: TricellMap::new(layout, Empty::INSTANCE, PlayerId::NONE, (PlayerId::NONE, false)),
            thief: Coord::ZERO,
            development_card: DevelopmentCards::new(),
            longest_road: PlayerId::NONE,
            largest_army: PlayerId::NONE,
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

    fn get_development_cards(&self) -> DevelopmentCards {
        self.development_card
    }

    fn get_development_cards_mut(&mut self) -> &mut DevelopmentCards {
        &mut self.development_card
    }

    fn get_thief_hex(&self) -> Coord {
        self.thief
    }

    fn set_thief_hex(&mut self, coord: Coord) {
        self.thief = coord
    }


    fn get_player_hand(&self, player: PlayerId) -> &PlayerHand {
        &self.players[player.to_u8() as usize]
    }

    fn get_player_hand_mut(&mut self, player: PlayerId) -> &mut PlayerHand {
        &mut self.players[player.to_u8() as usize]
    }

    fn get_longest_road(&self) -> Option<(PlayerId, u8)> {
        None
    }

    fn get_largest_army(&self) -> Option<(PlayerId, u8)> {
        None
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
    fn set_dynamic_path(&mut self, coord: Coord, player: PlayerId) -> Result<(), Error>{
        Ok(self.dynamic_board.set_path(coord, player)?)
    }

    fn get_dynamic_path(&self, coord: Coord) -> Result<Option<PlayerId>, Error>{
        let player = self.dynamic_board.get_path(coord)?;
        Ok(player.option())
    }

    fn set_dynamic_intersection(&mut self, coord: Coord, player: PlayerId, is_city: bool) -> Result<(), Error>{
        Ok(self.dynamic_board.set_intersection(coord, (player, is_city))?)
    }

    fn get_dynamic_intersection(&self, coord: Coord) -> Result<Option<(PlayerId, bool)>, Error>{
        let (player, is_city) = self.dynamic_board.get_intersection(coord)?;
        if player.to_u8() < self.player_count() {
            Ok(Some((player, is_city)))
        } else {
            Ok(None)
        }
    }
}
