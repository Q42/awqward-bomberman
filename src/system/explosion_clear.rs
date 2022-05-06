use bevy::{prelude::{Res, Query, Entity, Commands}, core::Time};

use crate::models::explosion::Explosion;

pub fn explosion_clear(mut commands: Commands, mut explosion_query: Query<(Entity, &mut Explosion)>, time: Res<Time>) {
    for (entity, mut explosion) in explosion_query.iter_mut() {
      explosion.0 -= time.delta().as_secs_f32();

      if explosion.0 <= 0.0 {
          commands.entity(entity).despawn();
      }
    }
}