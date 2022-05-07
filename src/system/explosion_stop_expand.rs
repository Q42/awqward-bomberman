use bevy::prelude::Commands;
use bevy::prelude::{Entity, Query, Res};
use bevy_rapier2d::plugin::RapierContext;

use crate::models::explosion::Explosion;
use crate::models::explosion::ExplosionDirection;
use crate::D;

use super::setup::Wall;

pub fn explosion_stop_expand(
    mut commands: Commands,
    wall_query: Query<(Entity, &Wall)>,
    mut explosion_query: Query<(Entity, &mut Explosion)>,
    rapier_context: Res<RapierContext>,
) {
    for (explosion_entity, mut explosion) in explosion_query.iter_mut() {
        for (wall_entity, wall) in wall_query.iter() {
            if rapier_context.intersection_pair(wall_entity, explosion_entity) == Some(true) {
                // If the wall type is a destructable wall, then we need to destroy it.
                if wall.0 == D {
                    commands.entity(wall_entity).despawn();
                }

                // Remove the explosion when we collide with a wall, then we will not expand further!
                use ExplosionDirection::*;
                explosion.1 = [None, None, None, None];
                commands.entity(explosion_entity).despawn();
            }
        }
    }
}
