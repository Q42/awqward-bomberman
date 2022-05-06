use bevy::prelude::Transform;
use bevy::prelude::{Res, Query, Commands};
use bevy_math::Vec3;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::RigidBody;
use bevy_rapier2d::prelude::Sensor;

use crate::models::atlas::Atlas;
use crate::models::explosion::ExplosionDirection;
use crate::models::explosion::Explosion;
use crate::system::explode_bomb::ExplosionBundle;

pub fn explosion_expand(mut commands: Commands, atlas: Res<Atlas>, explosion_query: Query<(&Explosion, &Transform)>, rapier_context: Res<RapierContext>) {
    for (explosion, transform) in explosion_query.iter() {
        for direction in explosion.0.iter() {
            use ExplosionDirection::*;
            match direction {
                Up => {
                    commands.spawn()
                    .insert_bundle(ExplosionBundle::new(atlas.handle.clone(), transform.with_translation(Vec3::new(transform.translation.x, transform.translation.y + 16.0, 0.0)), [Up, None, None, None]))
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::cuboid(16.0, 16.0))
                    .insert(Sensor(true));
                }
                Down => {
                    commands.spawn()
                    .insert_bundle(ExplosionBundle::new(atlas.handle.clone(), transform.with_translation(Vec3::new(transform.translation.x, transform.translation.y - 16.0, 0.0)), [Down, None, None, None]))
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::cuboid(16.0, 16.0))
                    .insert(Sensor(true));
                }
                Left => {
                    commands.spawn()
                    .insert_bundle(ExplosionBundle::new(atlas.handle.clone(), transform.with_translation(Vec3::new(transform.translation.x - 16.0, transform.translation.y, 0.0)), [Left, None, None, None]))
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::cuboid(16.0, 16.0))
                    .insert(Sensor(true));
                }
                Right => {
                    commands.spawn()
                    .insert_bundle(ExplosionBundle::new(atlas.handle.clone(), transform.with_translation(Vec3::new(transform.translation.x + 16.0, transform.translation.y, 0.0)), [Right, None, None, None]))
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::cuboid(16.0, 16.0))
                    .insert(Sensor(true));
                }
                None => { /* Do nothing */ }
            }
        }
    }
}