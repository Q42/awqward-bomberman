use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};

use crate::{models::player::Player, TIME_STEP};

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

pub fn check_for_collisions(
    mut player_query: Query<(&mut Velocity, &Transform), With<Player>>,
    collider_query: Query<&Transform, With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut player_velocity, player_transform) = player_query.single_mut();
    let player_size = player_transform.scale.truncate();

    for transform in collider_query.iter() {
        let collision = collide(
            player_transform.translation,
            player_size,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = player_velocity.x > 0.0,
                Collision::Right => reflect_x = player_velocity.x < 0.0,
                Collision::Top => reflect_y = player_velocity.y < 0.0,
                Collision::Bottom => reflect_y = player_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // Stop the x velocity if the player hits something on the x axis
            if reflect_x {
                player_velocity.x = 0.0;
            }

            // Stop the y velocity if the player hits something on the y axis
            if reflect_y {
                player_velocity.y = 0.0;
            }
        }
    }
}