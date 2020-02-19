use crate::state::{State, PlayerId};
use crate::player::Player;
use crate::utils::Resources;

use super::{Action, Phase};
use super::legal;

pub fn play_until_finished(phase: &mut Phase, state: &mut dyn State, player: &mut dyn Player) {
    player.new_game();
    loop {
        // If new turn, roll dice automatically
        if let Phase::Turn(_, false, _) = phase {
            apply(phase, state, Action::RollDice);
        }
        // If the game is finished, exit
        else if let Phase::FinishedGame(_) = phase {
            break;
        }

        let mut action;
        loop {
            // Ask player to take action
            action = player.pick_action(phase, state);
            if action == Action::Exit {
                return;
            }

            // Checks if action is legal
            let result = legal::legal(phase, state, action);
            if let Err(error) = result {
                // Tells player if action was invalid
                player.bad_action(error);
            } else {
                break;
            }
        }
        //Applies action
        apply(phase, state, action);
    }
}

/// Applies a legal action
///
/// Modifies a state by applying a given action, and/or changes the phase.action.
/// The function assumes that the action is legal and that it can be applied without problem.
/// It is necessary to call [legal](crate::game::legal::legal) beforehand to check if the action can indeed be applied without problem
fn apply(phase: &mut Phase, state: &mut dyn State, action: Action) {
    static ERROR_MESSAGE: &'static str = "Apply function failed because action supplied was illegal";
    let player = phase.player();
    match action {
        //
        // ## Ending Turn
        //
        Action::EndTurn => {
            *phase = Phase::Turn(PlayerId::from((player.to_u8() + 1) % state.player_count()),false,false);
        }
        //
        // ## Rolling Dice (Should be done automatically for now)
        //
        Action::RollDice => {
            // TODO: Roll dice and give resources
            // TODO: Handle thief
            if let Phase::Turn(_, dice_rolled, _) = phase {
                *dice_rolled = true;
            }
        }
        //
        // ## Building Road
        //
        Action::BuildRoad { path } => {
            state.get_player_hand_mut(player).road_pieces -= 1;
            state.set_dynamic_path(path, player).expect(ERROR_MESSAGE);
            if phase.is_turn() {
                state.get_player_hand_mut(player).resources -= Resources::ROAD;
                // TODO : Recompute longest road
            }
        }
        //
        // ## Building Settlement
        //
        Action::BuildSettlement { intersection } => {
            state.set_dynamic_intersection(intersection, player, false).expect(ERROR_MESSAGE);
            let mut hand = state.get_player_hand_mut(player);
            hand.settlement_pieces -= 1;
            hand.building_vp += 1;
            if phase.is_turn() {
                hand.resources -= Resources::SETTLEMENT;
            }
            // TODO : Recompute longest road if road broken
        }
        Action::BuildCity { intersection } => {
            state.set_dynamic_intersection(intersection, player, true).expect(ERROR_MESSAGE);
            let mut hand = state.get_player_hand_mut(player);
            hand.resources -= Resources::CITY;
            hand.settlement_pieces += 1;
            hand.city_pieces -= 1;
            hand.building_vp += 1;
        }

        Action::TradeBank { given, asked } => unimplemented!(),

        Action::BuyDvp => unimplemented!(),
        _ => unimplemented!(),
    }
    // TODO: Check if player just won
    // Special phase change if initial placement
    if let Phase::InitialPlacement(player, placing_second, placing_road) = phase {
        if !*placing_road {
            *placing_road = true
        } else {
            *placing_road = false;
            // If first placement
            if !*placing_second {
                if player.to_u8() == state.player_count() - 1 {
                    // If reached last player: switch to second placement
                    *placing_second = true;
                } else {
                    // Else change player clockwise
                    *player = PlayerId::from(player.to_u8() + 1);
                }
            // Else second placement
            } else {
                if *player == PlayerId::FIRST {
                    // If back to first player: switch to Turn-type phase
                    *phase = Phase::START_TURNS;
                } else {
                    // Else change player counter-clockwise
                    *player = PlayerId::from(player.to_u8() - 1);
                }
            }
        }
    }
}

/*
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
                            if player.to_u8() == state.player_count() - 1 {
                                // If reached last player: switch to second placement
                                *placing_second = true;
                            } else {
                                // Else change player clockwise
                                *player = PlayerId::from(player.to_u8() + 1);
                            }
                        // Else second placement
                        } else {
                            if *player == PlayerId::FIRST {
                                // If back to first player: switch to Turn-type phase
                                *phase = Phase::START_TURNS;
                            } else {
                                // Else change player counter-clockwise
                                *player = PlayerId::from(player.to_u8() - 1);
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
                    *phase = Phase::Turn(PlayerId::from((player.to_u8() + 1) % state.player_count()),false,false);
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
                    // ...the player has a road piece left...
                    && state.get_player_hand(*player).road_pieces >= 1
                    // ...and the player has enough resources for the road
                    && state.get_player_hand(*player).resources >= Resources::ROAD {

                    state.set_dynamic_path(path, *player)?;
                    state.get_player_hand_mut(*player).resources -= Resources::ROAD;
                    state.get_player_hand_mut(*player).road_pieces -= 1;
                    // TODO : Recompute longest road
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
                    let mut hand = state.get_player_hand_mut(*player);
                    hand.resources -= Resources::SETTLEMENT;
                    hand.settlement_pieces -= 1;
                    hand.building_vp += 1;
                    // TODO : Recompute longest road if road broken
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
                    let mut hand = state.get_player_hand_mut(*player);
                    hand.resources -= Resources::CITY;
                    hand.settlement_pieces += 1;
                    hand.city_pieces -= 1;
                    hand.building_vp += 1;
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
*/
