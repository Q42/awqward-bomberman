use crate::models::bomb::Bomb;
use bevy::log::info;
use bevy::prelude::*;

pub fn explode_bomb(
    mut commands: Commands,
    mut bomb_query: Query<(Entity, &mut Bomb)>,
    time: Res<Time>,
) {
    for (entity, mut bomb) in bomb_query.iter_mut() {
        bomb.remaining_time -= time.delta().as_secs_f32();

        if bomb.remaining_time <= 0.0 {
            info!("BOOOOOOOOOOOM MF");

            commands.entity(entity).despawn();
        }
    }
}
