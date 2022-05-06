use bevy::prelude::{Res, Query, Commands, Entity, With};
use bevy_rapier2d::plugin::RapierContext;

use crate::models::explosion::ExplosionDirection;
use crate::models::explosion::Explosion;

pub fn explosion_stop_expand(mut commands: Commands, wall_query: Query<(Entity, With<Wall>)>, mut explosion_query: Query<(Entity, &mut Explosion)>, rapier_context: Res<RapierContext>) {
    for wall_entity in wall_query.iter() {
        for (explosion_entity, explosion) in explosion_query.iter() {
            use ExplosionDirection::*;
            if rapier_context.intersection_pair(wall_entity, explosion_entity) == Some(true) {
                explosion.0 = [None, None, None, None];
            }
        }
    }
}