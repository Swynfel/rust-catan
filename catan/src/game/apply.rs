use rand::Rng;

use crate::state::{State, PlayerId};
use crate::utils::{Resource, Resources, Hex, LandHex, DevelopmentCard};
use crate::board::utils::topology::Topology;

use super::{Action, Phase, TurnPhase, DevelopmentPhase, Notification};

/// Applies a legal action
///
/// Modifies a state by applying a given action, and/or changes the phase.action.
/// The function assumes that the action is legal and that it can be applied without problem.
/// It is necessary to call [legal](crate::game::legal::legal) beforehand to check if the action can indeed be applied without problem
pub(super) fn apply<R : Rng>(phase: &mut Phase, state: &mut State, action: Action, rng: &mut R) -> Option<Notification> {
    static ERROR_MESSAGE: &'static str = "Apply function failed because action supplied was illegal";
    let player = phase.player();
    match action {
        //
        // ## Ending Turn
        //
        Action::EndTurn => {
            let hand = state.get_player_hand_mut(player);
            if hand.new_development_cards.total() > 0 {
                hand.development_cards += hand.new_development_cards;
                hand.new_development_cards.clear();
            }
            *phase = Phase::Turn {
                player: PlayerId::from((player.to_u8() + 1) % state.player_count()),
                turn_phase: TurnPhase::PreRoll,
                development_phase: DevelopmentPhase::Ready,
            };
        }
        //
        // ## Rolling Dice
        //
        Action::RollDice => {
            let roll = rng.gen_range(1, 7) + rng.gen_range(1, 7);
            if roll == 7 {
                // TODO: Discard cards
                for p in 0..state.player_count() {
                    let player = PlayerId::from(p);
                    let player_resources = state.get_player_hand(player).resources;
                    if player_resources.total() >= 7 {
                        for resource in Resource::ALL.iter() {
                            let resource_count = player_resources[*resource] / 2;
                            state.get_player_hand_mut(player).resources[*resource] -= resource_count;
                            state.get_bank_resources_mut()[*resource] += resource_count;
                        }
                    }
                }
                if let Phase::Turn { player: _, turn_phase, development_phase: _ } = phase {
                    *turn_phase = TurnPhase::MoveThief;
                }
                return Some(Notification::ThiefRolled);
            } else {
                let mut received_resources = vec![Resources::ZERO; state.player_count() as usize];
                let mut taken_resources = Resources::ZERO;
                // For each hex...
                for hex in state.get_layout().hexes.iter() {
                    // ...that produces resources...
                    if let Hex::Land(LandHex::Prod(res, num_token)) = state.get_static_hex(*hex).expect("Failed to inspect hex") {
                        // ..and has the correct number token and no thief on it...
                        if num_token == roll && *hex != state.get_thief_hex() {
                            // Look at every neighbour intersection...
                            for intersection in state.hex_intersection_neighbours(*hex).expect("Failed to inspect intersection") {
                                // ...with a settlement or city...
                                if let Some((player, is_city)) = state.get_dynamic_intersection(intersection).expect("Failed to inspect intersection") {
                                    // ...and add the resources to the corresponding player
                                    let r = if is_city {2} else {1};
                                    received_resources[player.to_usize()][res] += r;
                                    taken_resources[res] += r;
                                }
                            }
                        }
                    }
                }
                // Check that the bank has enough Resources
                let bank = state.get_bank_resources_mut();
                for res in Resource::ALL.iter() {
                    // If there is enough resource in the bank for everyone...
                    if bank[*res] >= taken_resources[*res] {
                        // ...remove them
                        bank[*res] -= taken_resources[*res];
                    } else {
                        let mut askers: Vec<&mut Resources> = received_resources.iter_mut()
                            .filter(|resources| resources[*res] > 0).collect();
                        // If there is only one player that requires the resource...
                        if askers.len() == 1 {
                            // ...give him what is left
                            askers[0][*res] = bank[*res];
                            bank[*res] = 0;
                        } else {
                            // ...no player gets anything
                            for asker in askers {
                                asker[*res] = 0;
                            }
                        }
                    }
                }
                // Then give the resources to the players
                for (i,resources) in received_resources.iter().enumerate() {
                    state.get_player_hand_mut(PlayerId::from(i as u8)).resources += *resources;
                }
                if let Phase::Turn { player: _, turn_phase, development_phase: _ } = phase {
                    *turn_phase = TurnPhase::Free;
                }
                return Some(Notification::ResourcesRolled { roll, resources: received_resources });
            }
        }
        //
        // ## Move Thief
        //
        Action::MoveThief { hex, victim } => {
            state.set_thief_hex(hex);
            if victim != player && victim != PlayerId::NONE {
                if state.get_player_hand(victim).resources.total() > 0 {
                    let mut resources = state.get_player_hand_mut(victim).resources;
                    let mut picked = rng.gen_range(0, resources.total());
                    for res in Resource::ALL.iter() {
                        if picked < resources[*res] {
                            resources[*res] -= 1;
                            state.get_player_hand_mut(player).resources[*res] += 1;
                            break;
                        } else {
                            picked -= resources[*res];
                        }
                    }
                }
            }
            if let Phase::Turn { player: _, turn_phase, development_phase } = phase {
                // If moved thief because of 7 roll
                if *turn_phase == TurnPhase::MoveThief {
                    *turn_phase = TurnPhase::Free;
                // Else moved thief because of knight development card
                } else {
                    *development_phase = DevelopmentPhase::DevelopmentPlayed;
                }
            }
        }
        //
        // ## Building Road
        //
        Action::BuildRoad { path } => {
            state.get_player_hand_mut(player).road_pieces -= 1;
            state.set_dynamic_path(path, player).expect(ERROR_MESSAGE);

            if let Phase::Turn {
                player:_,
                turn_phase: _,
                development_phase
            } = phase {
                // Spend free roads from Road Building Development Card
                if let DevelopmentPhase::RoadBuildingActive { two_left } = development_phase {
                    if *two_left {
                        *two_left = false;
                    } else {
                        *development_phase = DevelopmentPhase::DevelopmentPlayed;
                    }
                // Or spend resources
                } else {
                    state.get_player_hand_mut(player).resources -= Resources::ROAD;
                    *state.get_bank_resources_mut() += Resources::ROAD;
                };
            }

            state.update_longest_road(player, path);
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
                *state.get_bank_resources_mut() += Resources::SETTLEMENT;
            } else if *phase == (Phase::InitialPlacement { player, placing_second: true, placing_road: false }) {
                // Gives surrounding resources when placing the second settlement of the initial phase
                for hex in state.intersection_hex_neighbours(intersection).expect(ERROR_MESSAGE) {
                    if let Hex::Land(LandHex::Prod(res, _)) = state.get_static_hex(hex).expect(ERROR_MESSAGE) {
                        state.get_player_hand_mut(player).resources[res] += 1;
                        state.get_bank_resources()[res] -= 1;
                    }
                }
            }
            // Checks if an enemy road was broken
            let mut neighbour_players = vec![false; state.player_count() as usize];
            for path in state.intersection_path_neighbours(intersection).unwrap() {
                if let Some(p) = state.get_dynamic_path(path).unwrap() {
                    if p != player {
                        // If it's the p-player's second neighbour road
                        if neighbour_players[p.to_usize()] {
                            // Reset his longest road (in case it just got broken by this placement)
                            state.reset_longest_road(player);
                            // And exit, since there can only be one broken longest road per settlement
                            break;
                        // Else, if it's the first neighbour road
                        } else {
                            neighbour_players[p.to_usize()] = true;
                        }
                    }
                }
            }
        }
        //
        // ## Building City
        //
        Action::BuildCity { intersection } => {
            state.set_dynamic_intersection(intersection, player, true).expect(ERROR_MESSAGE);
            *state.get_bank_resources_mut() += Resources::CITY;
            let hand = state.get_player_hand_mut(player);
            hand.resources -= Resources::CITY;
            hand.settlement_pieces += 1;
            hand.city_pieces -= 1;
            hand.building_vp += 1;
        }
        //
        // ## Trade Bank
        //
        Action::TradeBank { given, asked } => {
            let hand = state.get_player_hand_mut(player);
            let given_count = hand.harbor.rate(given) as i8;
            hand.resources[given] -= given_count;
            hand.resources[asked] += 1;
            let bank = state.get_bank_resources_mut();
            bank[given] += given_count;
            bank[asked] -= 1;
        }
        //
        // ## Buy Development Card
        //
        Action::BuyDevelopment => {
            state.get_player_hand_mut(player).resources -= Resources::DVP_CARD;
            *state.get_bank_resources_mut() += Resources::DVP_CARD;
            let development = state.get_development_cards_mut();
            let mut picked = rng.gen_range(0, development.total());
            for dvp in DevelopmentCard::ALL.iter() {
                if picked < development[*dvp] {
                    // this development card was picked
                    development[*dvp] -= 1;
                    state.get_player_hand_mut(player).new_development_cards[*dvp] += 1;
                    break;
                } else {
                    picked -= development[*dvp];
                }
            }
        }
        //
        // ## Use Knight Development Card
        //
        Action::DevelopmentKnight => {
            state.get_player_hand_mut(player).development_cards.knight -= 1;
            state.get_player_hand_mut(player).knights += 1;
            if let Phase::Turn { player: _, turn_phase: _, development_phase } = phase {
                *development_phase = DevelopmentPhase::KnightActive;
            }
            state.update_largest_army(player);
        }
        //
        // ## Use Road Building Development Card
        //
        Action::DevelopmentRoadBuilding => {
            state.get_player_hand_mut(player).development_cards.road_building -= 1;
            if let Phase::Turn { player: _, turn_phase: _, development_phase } = phase {
                *development_phase = DevelopmentPhase::RoadBuildingActive { two_left: true };
            }
        }
        //
        // ## Use Year of Plenty Development Card
        //
        Action::DevelopmentYearOfPlenty => {
            state.get_player_hand_mut(player).development_cards.year_of_plenty -= 1;
            if let Phase::Turn { player: _, turn_phase: _, development_phase } = phase {
                *development_phase = DevelopmentPhase::YearOfPlentyActive { two_left: true };
            }
        }
        Action::ChooseFreeResource { resource } => {
            state.get_bank_resources_mut()[resource] -= 1;
            state.get_player_hand_mut(player).resources[resource] += 1;
            if let Phase::Turn { player: _, turn_phase: _, development_phase } = phase {
                if let DevelopmentPhase::YearOfPlentyActive { two_left } = development_phase {
                    if *two_left {
                        *two_left = false;
                    } else {
                        *development_phase = DevelopmentPhase::DevelopmentPlayed;
                    }
                }
            }
        }
        //
        // ## Use Year of Plenty Development Card
        //
        Action::DevelopmentMonopole { resource } => {
            state.get_player_hand_mut(player).development_cards.monopole -= 1;

            let mut total_taken = 0;
            for p in 0..state.player_count() {
                let p = PlayerId::from(p);
                if p != player {
                    total_taken += state.get_player_hand(p).resources[resource];
                    state.get_player_hand_mut(p).resources[resource] = 0;
                }
            }
            state.get_player_hand_mut(player).resources[resource] += total_taken;

            if let Phase::Turn { player: _, turn_phase: _, development_phase } = phase {
                *development_phase = DevelopmentPhase::DevelopmentPlayed;
            }
        }
        _ => unimplemented!(),
    }
    // Special phase change if initial placement
    if let Phase::InitialPlacement { player, placing_second, placing_road } = phase {
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
    // Check if a player just won
    } else {
        // We have to check every player because there is a (very rare) chance that another player just won
        // if we broke a third player's longest road and enabled this other player to get the longest road and go over 10 victory points
        for p in 0..state.player_count() {
            let player = PlayerId::from(p);
            if state.get_player_total_vp(player) >= 10 {
                *phase = Phase::FinishedGame { winner: player };
            }
        }
    }
    None
}
