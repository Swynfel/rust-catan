mod board;
mod state;
mod game;
mod utils;

use std::thread;

fn main() {
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
    }
}
