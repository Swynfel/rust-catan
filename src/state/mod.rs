mod display;
mod pretty_terminal_display;

pub use pretty_terminal_display::pretty_terminal_display;
use crate::utils::{Resource, Hex, CoordType, Harbor};
use crate::board::{map, Map as BoardMap, Layout as BoardLayout};

pub struct State<'a> {
    pub layout: &'a BoardLayout,
    pub board: BoardState<'a>,
    pub players: Vec<PlayerState>,
}

pub struct BoardState<'a> {
    pub layout: &'a BoardLayout,
    pub hexes: BoardMap<'a, Hex>,
    pub harbors: BoardMap<'a, Harbor>,
    pub dvp_card: u8,
}

pub struct PlayerState {
    pub resources: [i8; Resource::COUNT],
    pub dvp_carp_vp: u8,
}

impl BoardState<'_> {
    fn new(layout: &BoardLayout) -> BoardState {
        BoardState {
            layout,
            hexes: map::FullMap::new_typed(layout, CoordType::Hex),
            harbors: map::FullMap::new_typed(layout, CoordType::Intersection),
            dvp_card: 25,
        }
    }
}

impl State<'_> {
    pub fn new(layout: &BoardLayout, players: usize) -> State {
        State {
            layout,
            board: BoardState::new(layout),
            players: Vec::<PlayerState>::with_capacity(players),
        }
    }
}
