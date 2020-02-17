use crate::state::State;
use super::{Action, Error};
use crate::player::Player;

pub enum Phase {
    InitialPlacement(u8,bool,bool), // (player,placing_second,placing_road)
    Turn(u8,bool,bool), //(player,dice_rolled,dvp_card_played)
    FinishedGame(u8), //(winning_player)
}

pub fn play_until_finished(phase: &mut Phase, state: &mut State, player: &mut dyn Player) {
    let mut counter = 0;
    loop {
        counter += 1;
        let action = player.action_picker(phase, state);
        apply(phase, state, action).ok();
        if let Phase::FinishedGame(_) = phase {
            break;
        }
        if counter > 2 {
            break;
        }
    }
}

fn apply(phase: &mut Phase, state: &mut State, action: Action) -> Result<(), Error> {
    match phase {
        Phase::InitialPlacement(player, placing_second, placing_road) => {
            if *placing_road {
                if let Action::BuildRoad(coord) = action {
                    state.set_dynamic_path(coord, *player);
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
                if let Action::BuildSettlement(coord) = action {
                    if let Ok(value) = state.get_dynamic_intersection(coord) {
                        state.set_dynamic_intersection(coord, *player, false);
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
            Action::BuildRoad(coord) => unimplemented!(),
            Action::BuildSettlement(coord) => unimplemented!(),
            Action::BuildCity(coord) => unimplemented!(),

            Action::TradeBank(resource_from, resource_to) => unimplemented!(),

            Action::BuyDvp => unimplemented!(),
            _ => unimplemented!(),
        }
        _ => unimplemented!(),
    }
}

/*
pub fn legal(phase: &Phase, state: &State, action: &Action) -> bool {
    true
}
*/
