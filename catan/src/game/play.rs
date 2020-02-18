use crate::state::State;
use super::{Action, Error, Phase};
use crate::player::Player;

pub fn play_until_finished(phase: &mut Phase, state: &mut dyn State, player: &mut dyn Player) {
    loop {
        // If new turn, roll dice automatically
        if let Phase::Turn(_, false, _) = phase {
            apply(phase, state, Action::RollDice).expect("Couldn't automatically roll dice");
        }
        // If the game is finished, exit
        else if let Phase::FinishedGame(_) = phase {
            break;
        }

        // Ask player to take action
        let action = player.pick_action(phase, state);
        if action == Action::Exit {
            break;
        }
        // Applies action
        let result = apply(phase, state, action);
        if let Err(error) = result {
            // Tells player if action was invalid
            player.bad_action(error);
        }
    }
}

fn apply(phase: &mut Phase, state: &mut dyn State, action: Action) -> Result<(), Error> {
    match phase {
        Phase::InitialPlacement(player, placing_second, placing_road) => {
            if *placing_road {
                if let Action::BuildRoad { path } = action {
                    state.set_dynamic_path(path, *player)?;
                    /*** Changin Phase ***/
                    *placing_road = false;
                    // If first placement
                    if !*placing_second {
                        if *player == 0 {
                            // If reached last player: switch to second placement
                            *placing_second = true;
                        } else {
                            // Else change player clockwise
                            *player -= 1;
                        }
                    // Else second placement
                    } else {
                        if *player == 0 {
                            // If back to first player: switch to Turn-type phase
                            *phase = Phase::Turn(0,false,false);
                        } else {
                            // Else change player counter-clockwise
                            *player -= 1;
                        }
                    }
                    Ok(())
                } else {
                    Err(Error::IncoherentAction(action))
                }
            } else {
                if let Action::BuildSettlement { intersection } = action {
                    if let Ok(value) = state.get_dynamic_intersection(intersection) {
                        state.set_dynamic_intersection(intersection, *player, false)?;
                    } else {
                        return Err(Error::IncoherentAction(action))
                    };
                    /*** Changin Phase ***/
                    *placing_road = true;
                    Ok(())
                } else {
                    Err(Error::IncoherentAction(action))
                }
            }
        }
        Phase::Turn(player, dice_rolled, _) => match action {
            Action::EndTurn => {
                if *dice_rolled {
                    /*** Changin Phase ***/
                    *phase = Phase::Turn(*player+1 % state.player_count() as u8,false,false);
                    Ok(())
                } else {
                    Err(Error::IncoherentAction(action))
                }
            }
            Action::RollDice => {
                if *dice_rolled {
                    Err(Error::IncoherentAction(action))
                } else {
                    /*** Changin Phase ***/
                    *dice_rolled = true;
                    Ok(())
                }
            }
            Action::BuildRoad { path } => unimplemented!(),
            Action::BuildSettlement { intersection } => unimplemented!(),
            Action::BuildCity { intersection } => unimplemented!(),

            Action::TradeBank { given, asked } => unimplemented!(),

            Action::BuyDvp => unimplemented!(),
            _ => unimplemented!(),
        }
        _ => unimplemented!(),
    }
}
