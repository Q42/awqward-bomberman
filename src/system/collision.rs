use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::models::player::Player;

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

pub fn check_for_collisions(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    collider_query: Query<(&Transform, Option<&Player>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (player_transform) = player_query.single_mut();
    let player_size = player_transform.scale.truncate();

    // check collision with walls
    for (transform, maybe_player) in collider_query.iter() {
        let collision = collide(
            player_transform.translation,
            player_size,
            transform.translation,
            transform.scale.truncate(),
        );
        
        if collision.is_some() {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // Bricks should be despawned and increment the scoreboard on collision
            if maybe_player.is_some() {
                println!("player is colliding")
            }
        }
    }
}