mod display;
mod terminal_player;

use catan::board;
use catan::state::{State, TricellState};
use terminal_player::TerminalPlayer;
use catan::game::play::{self, Phase};

fn main() {
    println!("It's working?...");
    let mut state = board::setup::random_default::<TricellState>();

    let mut player = TerminalPlayer::new();
    play::play_until_finished(&mut Phase::Turn(0,false,false), &mut *state, &mut player);

    println!("Nope...");
}
