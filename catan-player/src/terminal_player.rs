use std::io::{stdout, Stdout, stdin, Write};
use termion::clear;
use termion::screen::AlternateScreen;

use catan::state::State;
use catan::player::Player;
use catan::game::Action;
use catan::game::play::Phase;

use crate::display::pretty_terminal;

pub struct TerminalPlayer {
    screen: AlternateScreen<Stdout>,
}

impl TerminalPlayer {
    pub fn new() -> TerminalPlayer {
        let screen = AlternateScreen::from(stdout());
        TerminalPlayer {
            screen,
        }
    }

    fn parse(raw: String) -> Option<Action> {
        None
    }
}

impl Player for TerminalPlayer {
    fn new_game(&mut self) {
        write!(self.screen, "{clear}", clear = clear::All).unwrap();
        writeln!(self.screen, "[New game]").unwrap();
        self.screen.flush().unwrap();
    }

    fn action_picker(&mut self, phase: &Phase, state: &dyn State) -> Action {
        pretty_terminal(&mut self.screen, state).unwrap();
        let mut action = None::<Action>;
        while action.is_none() {
            writeln!(self.screen, "Enter action:{clear}", clear = clear::AfterCursor).unwrap();
            let mut raw_action = String::new();
            stdin().read_line(&mut raw_action)
                .expect("Failed to read line");
            action = TerminalPlayer::parse(raw_action);
        }
        Action::EndTurn
    }
}
