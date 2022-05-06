use bevy::prelude::{Res, Query, With, Entity, Commands};
use bevy_rapier2d::plugin::RapierContext;

use crate::models::player::Player;

use super::explode_bomb::Explosion;

pub fn explosion_kill(mut commands: Commands, player_query: Query<Entity, With<Player>>, explosion_query: Query<Entity, With<Explosion>>, rapier_context: Res<RapierContext>) {
    for player_entity in player_query.iter() {
        for explosion_entity in explosion_query.iter() {
            if rapier_context.intersection_pair(player_entity, explosion_entity) == Some(true) {
                commands.entity(player_entity).despawn();
            }
        }
    }
}