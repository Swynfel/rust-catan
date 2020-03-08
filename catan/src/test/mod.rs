use crate::game::{Game, Notification};
use crate::state::PlayerId;
use crate::player::Randomy;

#[test]
fn play_random_game() {
   let mut game = Game::new();
   game.add_player(Box::new(Randomy::new_player()));
   game.add_player(Box::new(Randomy::new_player()));
   game.add_player(Box::new(Randomy::new_player()));
   game.add_player(Box::new(Randomy::new_player()));
   let notif = game.setup_and_play();
   assert_ne!(notif, Notification::GameFinished{ winner: PlayerId::NONE });
}
