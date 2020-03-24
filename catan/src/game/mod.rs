pub mod action;
mod error;
mod phase;
mod notification;
mod apply;
pub mod legal;

pub use error::Error;
pub use action::{Action, ActionCategory};
pub use phase::{Phase, TurnPhase, DevelopmentPhase};
pub use notification::Notification;

// --------------------------------------------------------------------------------------------- //

use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

use crate::state::{State, TricellState};
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

    pub fn setup_and_play(&mut self) -> Notification {
        let player_count = self.players.len();
        let mut rng = SmallRng::from_entropy();
        let mut state = setup::random_default::<TricellState, SmallRng>(&mut rng, player_count as u8);
        let mut players_order: Vec<usize> = (0..player_count).collect();
        players_order.shuffle(&mut rng);
        self.play(&mut rng, &mut state, players_order)
    }

    pub fn play(&mut self, rng: &mut SmallRng, state: &mut State, players_order: Vec<usize>) -> Notification {
        let mut phase = Phase::START_GAME;

        for (i, player) in players_order.iter().enumerate() {
            self.players[*player].new_game(PlayerId::from(i), &state);
        }
        loop {
            // If the game is finished, exit
            if let Phase::FinishedGame { winner } = phase {
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
            let prev_phase = phase;
            self.notify_all(Notification::ActionPlayed { by: phase.player(), action });
            // Applies action
            apply(&mut phase, state, action, rng);
            let coherence = check_coherence(state);
            if coherence.is_err() {
                println!("[INCOHERENCE] {:?} --({:?})-> {:?}", prev_phase, action, phase);
                panic!("{:?}", coherence.err());
            }
        }
    }
}

use crate::utils::{Resource, Resources};

fn check_coherence(state: &State) -> Result<(),String> {
    let mut players_resources = Resources::ZERO;
    for p in 0..state.player_count() {
        let player = PlayerId::from(p);
        let hand = state.get_player_hand(player).resources;
        for res in Resource::ALL.iter() {
            let v = hand[*res];
            if v > 19 || v < 0 {
                return Err(format!("Player {:?} has {} of {}", player, v, res));
            }
        }
        players_resources += hand;
    }
    let bank_resources = state.get_bank_resources();
    for res in Resource::ALL.iter() {
        let v = bank_resources[*res];
        let pv = players_resources[*res];
        if v > 19 || v < 0 || pv+v != 19 {
            return Err(format!("For resource {}: Bank has {} / Players have {}", res, v, pv));
        }
    }
    Ok(())
}
