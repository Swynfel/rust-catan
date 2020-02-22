use crate::utils::Resources;
use crate::game::Action;
use crate::state::PlayerId;

#[derive(Clone, Debug)]
pub enum Notification {
    ActionPlayed {
        by: PlayerId,
        action: Action,
    },
    ResourcesRolled {
        roll: u8,
        resources: Vec<Resources>,
    },
    GameFinished {
        winner: PlayerId,
    },
    ThiefRolled,
    InitialPlacementFinished,
}
