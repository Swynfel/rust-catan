use crate::state::State;
use crate::player::Player;
use crate::utils::Resources;

use super::{Action, Error, Phase};
use super::legal;

pub fn play_until_finished(phase: &mut Phase, state: &mut dyn State, player: &mut dyn Player) {
    player.new_game();
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
        //
        // # Initial Placement Phase
        //
        Phase::InitialPlacement(player, placing_second, placing_road) => {
            //
            // ## Building a Settlement
            //
            if !*placing_road {
                if let Action::BuildSettlement { intersection } = action {
                    if legal::available_settlement_position(intersection, state)? {
                        // Placing Settlement
                        state.set_dynamic_intersection(intersection, *player, false)?;
                        // Changing Phase
                        *placing_road = true;
                        Ok(())
                    } else {
                        Err(Error::IllegalAction(action))
                    }
                } else {
                    Err(Error::IncoherentAction(action))
                }
            //
            // ## Building a Road
            //
            } else {
                if let Action::BuildRoad { path } = action {
                    if legal::allowed_initial_road_placement(path, *player, state)? {
                        // We don't have to check it is unoccupied because the Settlement could not have been placed next to another existing road
                        state.set_dynamic_path(path, *player)?;
                        // Changing Phase
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
                                *phase = Phase::START_TURNS;
                            } else {
                                // Else change player counter-clockwise
                                *player -= 1;
                            }
                        }
                        Ok(())
                    } else {
                        Err(Error::IllegalAction(action))
                    }
                } else {
                    Err(Error::IncoherentAction(action))
                }
            }
        }
        //
        // # Regular Turn Phase
        //
        Phase::Turn(player, dice_rolled, _) => match action {
            //
            // ## Ending Turn
            //
            Action::EndTurn => {
                if *dice_rolled {
                    // Changing Phase
                    *phase = Phase::Turn(*player+1 % state.player_count() as u8,false,false);
                    Ok(())
                } else {
                    Err(Error::IncoherentAction(action))
                }
            }
            //
            // ## Rolling Dice (Should be done automatically for now)
            //
            Action::RollDice => {
                if *dice_rolled {
                    Err(Error::IncoherentAction(action))
                } else {
                    // Changing phase
                    *dice_rolled = true;
                    Ok(())
                }
            }
            //
            // ## Building Road
            //
            Action::BuildRoad { path } => {
                // If: we are next to a road...
                if legal::connected_position(path, *player, state)?
                    // ...the position is empty...
                    && state.get_dynamic_path(path)?.is_none()
                    // ...and the player has enough resources for the road
                    && state.get_player_hand(*player).resources >= Resources::ROAD {
                    state.set_dynamic_path(path, *player)?;
                    // TODO: Check pieces and remove one road
                    Ok(())
                } else {
                    Err(Error::IllegalAction(action))
                }
            }
            //
            // ## Building Settlement
            //
            Action::BuildSettlement { intersection } => {
                // If: we are next to a road...
                if legal::connected_position(intersection, *player, state)?
                    // ...the position is available (no settlement on it or next to it)...
                    && legal::available_settlement_position(intersection, state)?
                    // ...and the player has enough resources for the settlement
                    && state.get_player_hand(*player).resources >= Resources::SETTLEMENT {
                    state.set_dynamic_intersection(intersection, *player, false)?;
                    // TODO: Check pieces and remove one settlement
                    Ok(())
                } else {
                    Err(Error::IllegalAction(action))
                }
            }
            Action::BuildCity { intersection } => {
                // If: we already own a settlement at the position
                if Some((*player, false)) == state.get_dynamic_intersection(intersection)?
                    // ...and the player has enough resources for the city
                    && state.get_player_hand(*player).resources >= Resources::CITY {
                    state.set_dynamic_intersection(intersection, *player, true)?;
                    // TODO: Check pieces and add one settlement and remove one city
                    Ok(())
                } else {
                    Err(Error::IllegalAction(action))
                }
            }

            Action::TradeBank { given, asked } => unimplemented!(),

            Action::BuyDvp => unimplemented!(),
            _ => unimplemented!(),
        }
        _ => panic!("Game already finished"),
    }
}
