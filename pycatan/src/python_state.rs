use std::any::Any;
use ndarray::{Array1, Array3};

use catan::board::{Layout, Error};
use catan::utils::{Hex, LandHex, Harbor, Coord, DevelopmentCards, Resources};
use catan::state::PlayerHand;
use catan::state::{State, TricellState, StateTrait, StateMaker, PlayerId};

use super::PyObservationFormat;

pub struct PythonState {
    format: PyObservationFormat,
    player_count: usize,
    pub boards: Array1<Array3<i32>>,
    state: State,
}

impl PythonState {
    pub fn new(layout: &'static Layout, players: u8, format: PyObservationFormat) -> PythonState {
        let player_count = players as usize;
        PythonState {
            format,
            player_count,
            boards: vec![Array3::<i32>::zeros((format.width,format.height, 13 + 2 * players as usize)); player_count].into(),
            state: TricellState::new_empty(layout, players),
        }
    }

    pub fn set_all(&mut self, coord: Coord, channel: usize, value: i32)  {
        let x = (coord.x + self.format.half_width as i8) as usize;
        let y = (coord.y + self.format.half_height as i8) as usize;
        for board in self.boards.iter_mut() {
            board[(x, y, channel)] = value;
        }
    }

    pub fn set_roll(&mut self, coord: Coord, channel: usize, value: i32, player: PlayerId) {
        let x = (coord.x + self.format.half_width as i8) as usize;
        let y = (coord.y + self.format.half_height as i8) as usize;
        let mut i = player.to_usize();
        for board in self.boards.iter_mut() {
            board[(x, y, channel + i)] = value;
            if i == 0 {
                i = self.player_count - 1;
            } else {
                i -= 1;
            }
        }
    }
}

impl StateTrait for PythonState {
    fn get_layout(&self) -> &Layout { self.state.get_layout() }

    fn player_count(&self) -> u8 { self.state.player_count() }

    fn get_development_cards(&self) -> DevelopmentCards { self.state.get_development_cards() }

    fn get_development_cards_mut(&mut self) -> &mut DevelopmentCards { self.state.get_development_cards_mut() }

    fn get_bank_resources(&self) -> Resources { self.state.get_bank_resources() }

    fn get_bank_resources_mut(&mut self) -> &mut Resources { self.state.get_bank_resources_mut() }

    fn get_thief_hex(&self) -> Coord { self.state.get_thief_hex() }

    fn set_thief_hex(&mut self, coord: Coord) {
        self.set_all(self.state.get_thief_hex(), 6, 0);
        self.set_all(coord, 6, 1);
        self.state.set_thief_hex(coord);
    }

    fn hold_discards(&mut self, discards: Vec<(PlayerId, Option<Resources>)>) { self.state.hold_discards(discards) }

    fn peek_discards(&self) -> &Vec<(PlayerId, Option<Resources>)> { self.state.peek_discards() }

    fn set_discard(&mut self, player: PlayerId, resources: Resources) { self.state.set_discard(player, resources) }

    fn apply_discards(&mut self) { self.state.apply_discards() }

    // --- player related --- //

    fn get_player_hand(&self, player: PlayerId) -> &PlayerHand { self.state.get_player_hand(player) }

    fn get_player_hand_mut(&mut self, player: PlayerId) -> &mut PlayerHand { self.state.get_player_hand_mut(player) }

    fn get_longest_road(&self) -> Option<(PlayerId, u8)> { self.state.get_longest_road() }

    fn reset_longest_road(&mut self, player: PlayerId) { self.state.reset_longest_road(player) }

    fn update_longest_road(&mut self, player: PlayerId, root_path: Coord) { self.state.update_longest_road(player, root_path) }

    fn get_largest_army(&self) -> Option<(PlayerId, u8)> { self.state.get_largest_army() }

    fn update_largest_army(&mut self, player: PlayerId) { self.state.update_largest_army(player) }

    // --- static board --- //

    fn set_static_hex(&mut self, coord: Coord, hex: Hex) -> Result<(), Error> {
        self.state.set_static_hex(coord, hex)?;
        if let Hex::Land(land) = hex {
            match land {
                LandHex::Prod(res, value) => {
                    self.set_all(coord, res.to_usize(), value as i32);
                },
                LandHex::Desert => {
                    self.set_all(coord, 5, 1);
                },
            }
        }
        Ok(())
    }

    fn get_static_hex(&self, coord: Coord) -> Result<Hex, Error> { self.state.get_static_hex(coord) }

    fn set_static_harbor(&mut self, coord: Coord, harbor: Harbor) -> Result<(), Error> {
        self.state.set_static_harbor(coord, harbor)?;
        self.set_all(coord, 7 + self.player_count + harbor.to_usize(), 1);
        Ok(())
    }

    fn get_static_harbor(&self, coord: Coord) -> Result<Harbor, Error> { self.state.get_static_harbor(coord) }

    // --- dynamic board --- //

    fn set_dynamic_path(&mut self, coord: Coord, player: PlayerId) -> Result<(), Error>{
        self.state.set_dynamic_path(coord, player)?;
        self.set_roll(coord, 7, 1, player);
        Ok(())
    }

    fn get_dynamic_path(&self, coord: Coord) -> Result<Option<PlayerId>, Error> { self.state.get_dynamic_path(coord) }

    fn set_dynamic_intersection(&mut self, coord: Coord, player: PlayerId, is_city: bool) -> Result<(), Error>{
        self.state.set_dynamic_intersection(coord, player, is_city)?;
        self.set_roll(coord, 13 + self.player_count, if is_city { 2 } else { 1 }, player);
        Ok(())
    }

    fn get_dynamic_intersection(&self, coord: Coord) -> Result<Option<(PlayerId, bool)>, Error> { self.state.get_dynamic_intersection(coord) }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
