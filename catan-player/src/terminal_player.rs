use std::io::{stdout, Stdout, stdin, Write};
use termion::clear;
use termion::screen::AlternateScreen;

use catan::state::State;
use catan::player::Player;
use catan::game::{Action, Error, Phase};

use crate::display::{utils::grid_display, PrettyGridDisplay};
use super::action_parser::{parse_action, parse_help};

pub struct TerminalPlayer {
    screen: Stdout,
}

impl TerminalPlayer {
    pub fn new() -> TerminalPlayer {
        let screen = stdout(); //AlternateScreen::from(stdout());
        TerminalPlayer {
            screen,
        }
    }
}

impl Player for TerminalPlayer {
    fn new_game(&mut self) {
        write!(self.screen, "{clear}", clear = clear::All).unwrap();
        writeln!(self.screen, "[New game]").unwrap();
        self.screen.flush().unwrap();
    }

    fn pick_action(&mut self, phase: &Phase, state: &dyn State) -> Action {
        // Displays state
        grid_display(&PrettyGridDisplay::INSTANCE, &mut self.screen, state).expect("Failed to draw grid");
        writeln!(self.screen, "{:?}", phase).unwrap();
        loop {
            // Asks action
            writeln!(self.screen, "Enter action among:{help}{clear}", help = parse_help(), clear = clear::AfterCursor).unwrap();
            let mut raw_action = String::new();
            stdin().read_line(&mut raw_action)
                .expect("Failed to read line");
            match parse_action(raw_action) {
                // If wrong keyword passed, display error and try again
                Err(err) => {
                    writeln!(self.screen, "{:?}", err).expect("Failed to write error");
                }
                // Else, the action is correct so we can return it
                Ok(picked) => { return picked }
            }
        }
    }

    fn bad_action(&mut self, error: Error) {
        writeln!(self.screen, "{:?}", error).expect("Failed to write error");
    }
}
