use std::collections::VecDeque;
use std::io::{stdout, Stdout, stdin, Write};
use termion::clear;
//use termion::screen::AlternateScreen;

use catan::state::{State, PlayerId};
use catan::game::{Action, Error, Phase, Notification};
use catan::player::CatanPlayer;

use crate::display::utils::grid_display;
use crate::display::{PrettyGridDisplay, pretty_player_hand};
use super::action_parser::{parse_action, parse_help};

pub struct TerminalPlayer {
    screen: Stdout,
    bad_action: Option<Error>,
    notifications: VecDeque<Notification>,
}

impl TerminalPlayer {
    pub fn new() -> TerminalPlayer {
        let screen = stdout(); //AlternateScreen::from(stdout());
        TerminalPlayer {
            screen,
            bad_action: None,
            notifications: VecDeque::new(),
        }
    }

    pub fn write_state(&mut self, state: &State) {
        grid_display(&PrettyGridDisplay::INSTANCE, &mut self.screen, state).expect("Failed to draw grid");
        for i in 0..state.player_count() {
            let player = PlayerId::from(i);
            let hand = state.get_player_hand(player);
            pretty_player_hand(&mut self.screen, player, hand).expect("Failed to draw player hand");
            writeln!(&mut self.screen).expect("Failed to return line");
        }
    }

    pub fn write_notifications(&mut self) {
        for notification in self.notifications.iter() {
            writeln!(self.screen, "{:?}", notification).expect("Failed to write notification");
        }
    }

    pub fn write_error(&mut self) {
        if let Some(error) = self.bad_action {
            writeln!(self.screen, "[ERROR] {:?}", error).expect("Failed to write error");
        }
    }
}

impl CatanPlayer for TerminalPlayer {
    fn new_game(&mut self, _position: PlayerId, _: &State) {
        write!(self.screen, "{clear}", clear = clear::All).unwrap();
        writeln!(self.screen, "[New game]").unwrap();
        self.screen.flush().unwrap();
    }

    fn pick_action(&mut self, phase: &Phase, state: &State) -> Action {
        write!(self.screen, "{clear}", clear = clear::All).expect("Failed to clear screen");
        // Displays state
        self.write_state(state);
        // Displays notifications
        self.write_notifications();
        writeln!(self.screen, "{:?}", phase).unwrap();
        loop {
            // Displays previous error
            self.write_error();
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
                Ok(picked) => {
                    self.bad_action = None;
                    return picked;
                }
            }
        }
    }

    fn bad_action(&mut self, error: Error) {
        self.bad_action = Some(error);
    }

    fn notify(&mut self, notification: &Notification) {
        self.notifications.push_back(notification.clone());
        while self.notifications.len() > 8 {
            self.notifications.pop_front();
        }
    }

    fn results(&mut self, state: &State, winner: PlayerId) {
        self.write_state(state);
        writeln!(self.screen, "Winner : {:?}", winner).unwrap();
    }
}
