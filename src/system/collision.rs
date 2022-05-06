use bevy::{prelude::*};

use crate::models::player::Player;
use crate::utils::collide_aabb::{collide, Collision};

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

pub fn check_for_collisions(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (player_transform) = player_query.single_mut();
    let player_size = player_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, transform) in collider_query.iter() {
        let collision = collide(
            player_transform.translation,
            player_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            match collision {
                Collision::Left => {},
                Collision::Right => {},
                Collision::Top => {},
                Collision::Bottom => {},
                Collision::Inside => {}
            }
        }
    }
}