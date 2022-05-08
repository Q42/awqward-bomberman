use bevy::prelude::*;

use crate::model::explosion::Explosion;

pub fn explosion_clear(
    mut commands: Commands,
    mut explosion_query: Query<(Entity, &mut Explosion)>,
    time: Res<Time>,
) {
    for (entity, mut explosion) in explosion_query.iter_mut() {
        explosion.timer -= time.delta().as_secs_f32();

        if explosion.timer <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
