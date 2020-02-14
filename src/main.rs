mod board;
mod state;
mod constants;
mod error;
mod game;

fn main() {
    let g = game::generate_new_state();
    println!("Generated Catan Game");

    println!("{}", g.board);
}
