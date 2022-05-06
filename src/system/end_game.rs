use bevy::prelude::*;

use crate::models::player::Player;

pub fn end_game(_commands: Commands, player_query: Query<(Entity, &Player), With<Player>>) {
    if player_query.iter().count() != 1 {
        return;
    }

    let (_, winner) = player_query.iter().next().unwrap();
    info!("Winner!!!");
    dbg!(winner);
}
