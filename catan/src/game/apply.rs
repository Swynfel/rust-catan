use rand::Rng;

use crate::state::{State, PlayerId};
use crate::utils::{Resources, Hex, LandHex, DevelopmentCard};
use crate::board::utils::topology::Topology;

use super::{Action, Phase, Notification};

/// Applies a legal action
///
/// Modifies a state by applying a given action, and/or changes the phase.action.
/// The function assumes that the action is legal and that it can be applied without problem.
/// It is necessary to call [legal](crate::game::legal::legal) beforehand to check if the action can indeed be applied without problem
pub(super) fn apply<R : Rng>(phase: &mut Phase, state: &mut dyn State, action: Action, rng: &mut R) -> Option<Notification> {
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
            let roll = rng.gen_range(1, 7) + rng.gen_range(1, 7);
            if roll == 7 {
                // TODO: Handle thief
                if let Phase::Turn(_, dice_rolled, _) = phase {
                    *dice_rolled = true;
                }
                return Some(Notification::ThiefRolled);
            } else {
                let mut received_resources = vec![Resources::ZERO; state.player_count() as usize];
                // For each hex...
                for hex in state.get_layout().hexes.iter() {
                    // ...that produces resources...
                    if let Hex::Land(LandHex::Prod(res, num_token)) = state.get_static_hex(*hex).expect("Failed to inspect hex") {
                        // ..and has the correct number token
                        if num_token == roll {
                            // Look at every neighbour intersection...
                            for intersection in state.hex_intersection_neighbours(*hex).expect("Failed to inspect intersection") {
                                // ...with a settlement or city...
                                if let Some((player, is_city)) = state.get_dynamic_intersection(intersection).expect("Failed to inspect intersection") {
                                    // ...and add the resources to the corresponding player
                                    received_resources[player.to_usize()][res] += if is_city {2} else {1};
                                }
                            }
                        }
                    }
                }
                // Then give the resources to the players
                for (i,resources) in received_resources.iter().enumerate() {
                    state.get_player_hand_mut(PlayerId::from(i as u8)).resources += *resources;
                }
                if let Phase::Turn(_, dice_rolled, _) = phase {
                    *dice_rolled = true;
                }
                return Some(Notification::ResourcesRolled { roll, resources: received_resources });
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
            let harbor = state.get_static_harbor(intersection).expect(ERROR_MESSAGE);
            let hand = state.get_player_hand_mut(player);
            hand.settlement_pieces -= 1;
            hand.building_vp += 1;
            hand.harbor.add(harbor);
            if phase.is_turn() {
                hand.resources -= Resources::SETTLEMENT;
            } else if *phase == Phase::InitialPlacement(player, true, false) {
                // Gives surrounding resources when placing the second settlement of the initial phase
                for hex in state.intersection_hex_neighbours(intersection).expect(ERROR_MESSAGE) {
                    if let Hex::Land(LandHex::Prod(res, _)) = state.get_static_hex(hex).expect(ERROR_MESSAGE) {
                        state.get_player_hand_mut(player).resources[res] += 1;
                    }
                }
            }
            // TODO : Recompute longest road if road broken
        }
        Action::BuildCity { intersection } => {
            state.set_dynamic_intersection(intersection, player, true).expect(ERROR_MESSAGE);
            let hand = state.get_player_hand_mut(player);
            hand.resources -= Resources::CITY;
            hand.settlement_pieces += 1;
            hand.city_pieces -= 1;
            hand.building_vp += 1;
        }

        Action::TradeBank { given, asked } => {
            let hand = state.get_player_hand_mut(player);
            hand.resources[given] -= hand.harbor.rate(given) as i8;
            hand.resources[asked] += 1;
        }

        Action::BuyDevelopment => {
            state.get_player_hand_mut(player).resources -= Resources::DVP_CARD;
            let development = state.get_development_cards_mut();
            let mut picked = rng.gen_range(0, development.total());
            for dvp in DevelopmentCard::ALL.iter() {
                if picked < development[*dvp] {
                    // this development card was picked
                    development[*dvp] -= 1;
                    state.get_player_hand_mut(player).development_cards[*dvp] += 1;
                    break;
                } else {
                    picked -= development[*dvp];
                }
            }
        }
        _ => unimplemented!(),
    }
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
    // Check if player just won
    } else if state.get_player_total_vp(player) >= 10 {
        *phase = Phase::FinishedGame(player);
    }
    None
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

            Action::BuyDevelopment => unimplemented!(),
            _ => unimplemented!(),
        }
        _ => panic!("Game already finished"),
    }
}
*/
