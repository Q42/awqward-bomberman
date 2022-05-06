use bevy::prelude::{Res, Query, Entity, With};
use bevy_rapier2d::plugin::RapierContext;

use crate::models::explosion::ExplosionDirection;
use crate::models::explosion::Explosion;

use super::setup::Wall;

pub fn explosion_stop_expand(wall_query: Query<(Entity, With<Wall>)>, mut explosion_query: Query<(Entity, &mut Explosion)>, rapier_context: Res<RapierContext>) {
    for (wall_entity, _wall) in wall_query.iter() {
        for (explosion_entity, mut explosion) in explosion_query.iter_mut() {
            use ExplosionDirection::*;
            if rapier_context.intersection_pair(wall_entity, explosion_entity) == Some(true) {
                explosion.1 = [None, None, None, None];
            }
        }
    }
}