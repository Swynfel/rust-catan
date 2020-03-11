use std::any::Any;

use crate::board::map::TricellMap;
use crate::board::{Layout, Error};
use crate::utils::{Empty, Hex, Harbor, Coord, DevelopmentCards, Resources};
use crate::board::utils::topology::Topology;
use super::PlayerHand;
use super::{State, StateTrait, StateMaker, PlayerId};

pub struct TricellState {
    layout: &'static Layout,
    static_board: Box<TricellMap<Hex,Empty,Harbor>>,
    dynamic_board: Box<TricellMap<Empty,PlayerId,(PlayerId,bool)>>,
    thief: Coord,
    development_card: DevelopmentCards,
    longest_road: PlayerId,
    largest_army: PlayerId,
    players: Vec<PlayerHand>,
    bank_resources: Resources,
}

impl TricellState {
    pub fn new(layout: &'static Layout, players: usize) -> TricellState {
        TricellState {
            layout,
            static_board: TricellMap::new(&layout, Hex::Water, Empty::INSTANCE, Harbor::None),
            dynamic_board: TricellMap::new(&layout, Empty::INSTANCE, PlayerId::NONE, (PlayerId::NONE, false)),
            thief: Coord::ZERO,
            development_card: DevelopmentCards::new(),
            longest_road: PlayerId::NONE,
            largest_army: PlayerId::NONE,
            players: vec![PlayerHand::new();players],
            bank_resources: Resources::STARTING_BANK,
        }
    }

    /// Returns a (path, next_intersection) vector with the potential next paths
    /// No path is returned if 'intersection' is an enemy settlement or city
    /// The returned paths are connected to 'intersection' and aren't any path from 'used_paths'
    fn next_chain_paths(&self, player: PlayerId, used_paths: &Vec<Coord>, intersection: Coord) -> Vec<(Coord, Coord)> {
        // If another player occupies the intersection, interupt pathchain
        if let Some((p, _)) = self.get_dynamic_intersection(intersection).unwrap() {
            if player != p {
                return Vec::new();
            }
        }
        let mut paths = Vec::new();
        // Check every paths connected to the current intersection
        for path in self.intersection_path_neighbours(intersection).unwrap() {
            // If the path has a player's road placed on it...
            if let Some(p) = self.get_dynamic_path(path).unwrap() {
                // ...and this player is the correct one...
                // ...and the road hasn't been used yet...
                if player == p && !used_paths.contains(&path) {
                    // ...add this road as a potential path:
                    // finds the intersection that is connected to the path that isn't the current_intersection
                    for next_intersection in self.path_intersection_neighbours(path).unwrap() {
                        if next_intersection != intersection {
                            paths.push((path, next_intersection));
                        }
                    }
                }
            }
        }
        paths
    }

    /// Recursive function that returns the longest chain that has 'chain' as a sub-chain
    fn longest_chain(&self, player: PlayerId, mut chain: PathChain) -> usize {
        if let Some(head_intersection) = chain.head {
            let nexts = self.next_chain_paths(player, &chain.paths, head_intersection);
            if nexts.len() == 0 {
                chain.head = None
            } else {
                let mut length = 0;
                for (next_path, next_intersection) in nexts {
                    let mut paths = chain.paths.clone();
                    paths.push(next_path);
                    let r = self.longest_chain(player,
                        PathChain {
                            paths,
                            head: Some(next_intersection),
                            tail: chain.tail,
                        }
                    );
                    if r > length {
                        length = r;
                    }
                }
                return length;
            }
        }
        if let Some(tail_intersection) = chain.tail {
            let nexts = self.next_chain_paths(player, &chain.paths, tail_intersection);
            if nexts.len() > 0 {
                let mut length = 0;
                for (next_path, next_intersection) in nexts {
                    let mut paths = chain.paths.clone();
                    paths.push(next_path);
                    let r = self.longest_chain(player,
                        PathChain {
                            paths,
                            head: chain.head,
                            tail: Some(next_intersection),
                        }
                    );
                    if r > length {
                        length = r;
                    }
                }
                return length;
            }
        }
        chain.paths.len()
    }
}

struct PathChain {
    paths: Vec<Coord>,
    head: Option<Coord>,
    tail: Option<Coord>,
}

impl StateMaker for TricellState {
    fn new_empty(layout: &'static Layout, player_count: u8) -> State {
        Box::new(TricellState::new(layout, player_count as usize))
    }
}

impl StateTrait for TricellState {
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

    fn get_bank_resources(&self) -> Resources {
        self.bank_resources
    }

    fn get_bank_resources_mut(&mut self) -> &mut Resources {
        &mut self.bank_resources
    }

    fn get_thief_hex(&self) -> Coord {
        self.thief
    }

    fn set_thief_hex(&mut self, coord: Coord) {
        self.thief = coord
    }

    // --- player related --- //

    fn get_player_hand(&self, player: PlayerId) -> &PlayerHand {
        &self.players[player.to_u8() as usize]
    }

    fn get_player_hand_mut(&mut self, player: PlayerId) -> &mut PlayerHand {
        &mut self.players[player.to_u8() as usize]
    }

    fn get_longest_road(&self) -> Option<(PlayerId, u8)> {
        if self.longest_road == PlayerId::NONE {
            None
        } else {
            Some((self.longest_road, self.get_player_hand(self.longest_road).continous_road))
        }
    }

    // TODO: Try to optimise this function a little more
    // Some paths are explored about number_of_roads to many times
    fn reset_longest_road(&mut self, player: PlayerId) {
        let paths = self.get_layout().paths.clone();
        for path in paths {
            if let Some(p) = self.get_dynamic_path(path).unwrap() {
                if player == p {
                    self.update_longest_road(player, path)
                }
            }
        }
    }

    fn update_longest_road(&mut self, player: PlayerId, root_path: Coord) {
        let old_length = self.get_player_hand(player).continous_road;

        let intersections = self.path_intersection_neighbours(root_path).unwrap();
        let new_length = self.longest_chain(player,
            PathChain {
                paths: vec![root_path],
                head: Some(intersections[0]),
                tail: Some(intersections[1]),
            }
        ) as u8;

        if new_length > old_length {
            self.get_player_hand_mut(player).continous_road = new_length;
        }
        if new_length < 5 {
            return;
        }
        for (i, hand) in self.players.iter().enumerate() {
            if i != player.to_usize() && new_length <= hand.continous_road {
                return;
            }
        }
        self.longest_road = player;
    }

    fn get_largest_army(&self) -> Option<(PlayerId, u8)> {
        if self.largest_army == PlayerId::NONE {
            None
        } else {
            Some((self.largest_army, self.get_player_hand(self.largest_army).knights))
        }
    }

    fn update_largest_army(&mut self, player: PlayerId) {
        let size = self.get_player_hand(player).knights;
        if size < 3 {
            return;
        }
        for (i, hand) in self.players.iter().enumerate() {
            if i != player.to_usize() && size <= hand.knights {
                return;
            }
        }
        self.largest_army = player;
    }

    // --- static board --- //

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

    // --- dynamic board --- //

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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
