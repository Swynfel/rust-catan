use crate::state::PlayerId;

/// Changes the PlayerId to a position relative to the player
///
/// 0 will be the player, 1 will be the next one to play after the player, 2 will be the second one, etc...
pub fn player_id_to_relative(player: PlayerId, id: PlayerId, player_count: u8) -> PlayerId {
    let relative = player_count + id.to_u8() - player.to_u8();
    if relative >= player_count {
        PlayerId::from(relative - player_count)
    } else {
        PlayerId::from(relative)
    }
}

/// Changes a position relative to the player to a PlayerId
///
/// See [player_id_to_relative]
pub fn relative_to_player_id(player: PlayerId, relative: PlayerId, player_count: u8) -> PlayerId {
    offset_to_player_id(player, relative.to_u8(), player_count)
}

/// Changes a position relative to the player to a PlayerId
///
/// See [player_id_to_relative]
pub fn offset_to_player_id(player: PlayerId, relative: u8, player_count: u8) -> PlayerId {
    let id = player.to_u8() + relative;
    if id >= player_count {
        PlayerId::from(id - player_count)
    } else {
        PlayerId::from(id)
    }
}
