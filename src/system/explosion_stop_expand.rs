use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::model::destructable::Destructable;
use crate::model::explosion::{Explosion, FINISHED_EXPLOSION};
use crate::{D, G};

use super::setup::Wall;

pub fn explosion_stop_expand(
    mut commands: Commands,
    mut wall_query: Query<(Entity, &mut Wall, &mut TextureAtlasSprite)>,
    mut explosion_query: Query<(Entity, &mut Explosion)>,
    rapier_context: Res<RapierContext>,
) {
    for (explosion_entity, mut explosion) in explosion_query.iter_mut() {
        for (wall_entity, mut wall, mut texture_atlas) in wall_query.iter_mut() {
            if rapier_context.intersection_pair(wall_entity, explosion_entity) == Some(true) {
                // If the wall type is a destructable wall, then we need to destroy it.
                if wall.0 == D {
                    wall.0 = G;
                    texture_atlas.index = G;
                    commands
                        .entity(wall_entity)
                        .remove::<Destructable>()
                        .remove::<Wall>()
                        .remove::<RigidBody>()
                        .remove::<Collider>();
                }

                // Remove the explosion when we collide with a wall, then we will not expand further!
                explosion.directions = FINISHED_EXPLOSION;
            }
        }
    }
}
