use bevy::prelude::Commands;
use bevy::prelude::{Res, Query, Entity};
use bevy_rapier2d::plugin::RapierContext;

use crate::models::explosion::Explosion;
use crate::models::explosion::ExplosionDirection;

use super::setup::Wall;

pub fn explosion_stop_expand(mut commands: Commands, wall_query: Query<(Entity, &Wall)>, mut explosion_query: Query<(Entity, &mut Explosion)>, rapier_context: Res<RapierContext>) {
    for (explosion_entity, mut explosion) in explosion_query.iter_mut() {
        for (wall_entity, _wall) in wall_query.iter() {
            if rapier_context.intersection_pair(wall_entity, explosion_entity) == Some(true) {
                use ExplosionDirection::*;
                explosion.1 = [None, None, None, None];
                commands.entity(explosion_entity).despawn();
            }
        }
    }
}