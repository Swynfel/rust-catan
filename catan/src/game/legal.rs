use crate::utils::{Coord, CoordType, Resources, Harbor};
use crate::state::{State, PlayerId};
use crate::game::{Phase, Action, Error};
use crate::board::utils::topology::Topology;
use crate::board::Error as BoardError;

/// Is the intersection free for a settlement
///
/// Returns true if there is no settlement at the coord or around it
pub fn available_settlement_position(coord: Coord, state: &dyn State) -> Result<bool, BoardError> {
    let neighbours = state.intersection_intersection_neighbours(coord)?;
    for neighbour_intersection in neighbours {
        if state.get_dynamic_intersection(neighbour_intersection)?.is_some() {
            return Ok(false);
        }
    }
    return Ok(state.get_dynamic_intersection(coord)?.is_none());
}

/// Is this position allowed for a inital placement road
///
/// Returns true if the path or intersection coord is next to a road owned by the player
pub fn allowed_initial_road_placement(coord: Coord, player: PlayerId, state: &dyn State) -> Result<bool, BoardError> {
    let neighbours = state.path_intersection_neighbours(coord)?;
    let mut neighbour_settlement = None;
    for neighbour in neighbours {
        if let Some((p, _)) = state.get_dynamic_intersection(neighbour)? {
            if player == p {
                neighbour_settlement = Some(neighbour);
            }
        }
    }
    if let Some(neighbour_settlement) = neighbour_settlement {
        let connected = connected_position(neighbour_settlement, player, state)?;
        // If the settlement is already connected it means we are putting the player is placing the road next to the wrong selltement
        Ok(!connected)
    } else {
        Ok(false)
    }
}

/// Is the path or intersection connected to a piece owned by the player
///
/// Returns true if the path or intersection coord is next to a road owned by the player
pub fn connected_position(coord: Coord, player: PlayerId, state: &dyn State) -> Result<bool, BoardError> {
    let neighbours = match coord.get_type() {
        CoordType::Path => state.path_path_neighbours(coord)?,
        CoordType::Intersection => state.intersection_path_neighbours(coord)?,
        t => return Err(BoardError::MultiWrongCoordType { expected:[false, false, true, true], received:t }),
    };
    for neighbour in neighbours {
        if let Some(p) = state.get_dynamic_path(neighbour)? {
            if player == p {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Is the action legal in this context
///
/// Returns either an ok if the action can be played in the current phase and state,
/// or an Error describing why the action can't be played
pub fn legal(phase: &Phase, state: &dyn State, action: Action) -> Result<(), Error> {
    match phase {
        //
        // # Initial Placement Phase
        //
        Phase::InitialPlacement(player, _, placing_road) => {
            //
            // ## Building a Settlement
            //
            if !*placing_road {
                if let Action::BuildSettlement { intersection } = action {
                    // If the position is available
                    if available_settlement_position(intersection, state)? {
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
                    if allowed_initial_road_placement(path, *player, state)? {
                        // We don't have to check it is unoccupied because the Settlement could not have been placed next to another existing road
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
                    Ok(())
                }
            }
            //
            // ## Building Road
            //
            Action::BuildRoad { path } => {
                // If we are next to a road...
                if connected_position(path, *player, state)?
                    // ...the position is empty...
                    && state.get_dynamic_path(path)?.is_none()
                    // ...the player has a road piece left...
                    && state.get_player_hand(*player).road_pieces >= 1
                    // ...and the player has enough resources for the road
                    && state.get_player_hand(*player).resources >= Resources::ROAD {
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
                if connected_position(intersection, *player, state)?
                    // ...the position is available (no settlement on it or next to it)...
                    && available_settlement_position(intersection, state)?
                    // ...the player has a settlement piece left...
                    && state.get_player_hand(*player).settlement_pieces >= 1
                    // ...and the player has enough resources for the settlement
                    && state.get_player_hand(*player).resources >= Resources::SETTLEMENT {
                    Ok(())
                } else {
                    Err(Error::IllegalAction(action))
                }
            }
            Action::BuildCity { intersection } => {
                // If: we already own a settlement at the position
                if Some((*player, false)) == state.get_dynamic_intersection(intersection)?
                    // ...and the player has a city piece left...
                    && state.get_player_hand(*player).city_pieces >= 1
                    // ...and the player has enough resources for the city
                    && state.get_player_hand(*player).resources >= Resources::CITY {
                    Ok(())
                } else {
                    Err(Error::IllegalAction(action))
                }
            }

            Action::TradeBank { given, asked } => {
                if given == asked {
                    return Err(Error::IllegalTradeSameResources(given));
                }
                let hand = state.get_player_hand(*player);
                let rate = hand.harbor.rate(given);
                if hand.resources[given] < rate as i8 {
                    Err(Error::NotEnoughResources { required: Resources::new_one(given, rate as i8), have: hand.resources })
                } else {
                    Ok(())
                }
            }

            Action::BuyDvp => {
                if state.get_dvp_card_left() >= 1
                    && state.get_player_hand(*player).resources >= Resources::DVP_CARD {
                        Ok(())
                } else {
                    Err(Error::IllegalAction(action))
                }
            }
            _ => unimplemented!(),
        }
        _ => panic!("Game already finished"),
    }
}
