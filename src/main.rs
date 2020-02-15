mod board;
mod state;
mod game;
mod utils;

use std::io::stdout;
use crate::state::pretty_terminal_display;
use termion::{color, cursor, clear, style};

fn main() {
    println!("{clear}", clear = clear::All);
    pretty_terminal_display(&mut stdout(), &board::setup::random_default().board).unwrap();

    /*
    let mut threads = Vec::new();
    for _ in 0..10 {
        threads.push(
            thread::spawn( ||
                println!("{}", board::setup::random_default().board)
            )
        );
    }
    for thread in threads {
        thread.join().unwrap();
    }*/
}
