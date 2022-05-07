use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::model::{explosion::Explosion, player::Player};

pub fn explosion_kill(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    explosion_query: Query<Entity, With<Explosion>>,
    rapier_context: Res<RapierContext>,
) {
    for player_entity in player_query.iter() {
        for explosion_entity in explosion_query.iter() {
            if rapier_context.intersection_pair(player_entity, explosion_entity) == Some(true) {
                commands.entity(player_entity).despawn();
            }
        }
    }
}
