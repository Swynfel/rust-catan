pub mod action;
mod error;
mod phase;
mod notification;
mod apply;
pub mod legal;

pub use error::Error;
pub use action::{Action, ActionCategory};
pub use phase::{Phase, TurnPhase};
pub use notification::Notification;

// --------------------------------------------------------------------------------------------- //

use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

use crate::state::TricellState;
use crate::board::setup;
use crate::state::PlayerId;
use crate::player::CatanPlayer;

use apply::apply;

pub struct Game {
    pub players: Vec<Box<dyn CatanPlayer>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Box<dyn CatanPlayer>) {
        self.players.push(player);
    }

    fn notify_all(&mut self, notification: Notification) {
        for player in self.players.iter_mut() {
            player.notify(&notification);
        }
    }

    pub fn play(&mut self) -> Notification {
        let player_count = self.players.len();
        let mut rng = SmallRng::from_entropy();
        let mut state = setup::random_default::<TricellState, SmallRng>(&mut rng, player_count as u8);
        let mut players_order: Vec<usize> = (0..player_count).collect();
        players_order.shuffle(&mut rng);

        let mut phase = Phase::START_GAME;

        for (i, player) in players_order.iter().enumerate() {
            self.players[*player].new_game(PlayerId::from(i), &state);
        }
        loop {
            // If new turn, roll dice automatically
            if let Phase::Turn(_, TurnPhase::PreRoll, _) = phase {
                if let Some(notification) = apply(&mut phase, &mut state, Action::RollDice, &mut rng) {
                    self.notify_all(notification);
                }
            }
            // If the game is finished, exit
            else if let Phase::FinishedGame(winner) = phase {
                for player in players_order.iter() {
                    self.players[*player].results(&state, winner);
                }
                return Notification::GameFinished { winner };
            }

            // Get the player object that is supposed to be making a decision
            let player = &mut self.players[players_order[phase.player().to_u8() as usize]];
            let mut action;
            loop {
                // Ask player to take action
                action = player.pick_action(&phase, &state);
                if action == Action::Exit {
                    return Notification::GameFinished { winner: PlayerId::NONE };
                }

                // Checks if action is legal
                let result = legal::legal(&phase, &state, action);
                if let Err(error) = result {
                    // Tells player if action was invalid
                    player.bad_action(error);
                } else {
                    break;
                }
            }

            // Notifies every player of action played
            self.notify_all(Notification::ActionPlayed { by: phase.player(), action });
            // Applies action
            apply(&mut phase, &mut state, action, &mut rng);
        }
    }
}
