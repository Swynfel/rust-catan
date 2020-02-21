pub mod action;
mod error;
mod phase;
mod notification;
mod apply;
pub mod legal;

pub use error::Error;
pub use action::Action;
pub use phase::Phase;
pub use notification::Notification;

// --------------------------------------------------------------------------------------------- //

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::state::TricellState;
use crate::board::setup;
use crate::player::Player;

use apply::apply;

pub struct Game {
    pub players: Vec<Box<dyn Player>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Box<dyn Player>) {
        self.players.push(player);
    }

    pub fn play(&mut self) {
        let player_count = self.players.len();
        let mut state = setup::random_default::<TricellState>(player_count as u8);
        let mut players_order: Vec<usize> = (0..player_count).collect();
        players_order.shuffle(&mut thread_rng());

        let mut phase = Phase::START_GAME;

        for (i, player) in players_order.iter().enumerate() {
            self.players[*player].new_game(i as u8, &*state);
        }
        loop {
            // If new turn, roll dice automatically
            if let Phase::Turn(_, false, _) = phase {
                apply(&mut phase, &mut *state, Action::RollDice);
            }
            // If the game is finished, exit
            else if let Phase::FinishedGame(winner) = phase {
                for player in players_order.iter() {
                    self.players[*player].notify(Notification::GameFinished { winner });
                }
                break;
            }

            // Get the player object that is supposed to be making a decision
            let player = &mut self.players[players_order[phase.player().to_u8() as usize]];
            let mut action;
            loop {
                // Ask player to take action
                action = player.pick_action(&phase, &*state);
                if action == Action::Exit {
                    return;
                }

                // Checks if action is legal
                let result = legal::legal(&phase, &*state, action);
                if let Err(error) = result {
                    // Tells player if action was invalid
                    player.bad_action(error);
                } else {
                    break;
                }
            }

            // Notifies every player of action played
            for player in players_order.iter() {
                self.players[*player].notify(Notification::ActionPlayed { by: phase.player(), action });
            }
            // Applies action
            apply(&mut phase, &mut *state, action);
        }
    }
}
