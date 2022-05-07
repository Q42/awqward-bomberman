use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_math::Vec3;
use bevy_rapier2d::prelude::*;

use crate::model::atlas::Atlas;
use crate::model::explosion::ExplosionDirection;
use crate::model::explosion::{Explosion, FINISHED_EXPLOSION};
use crate::system::explode_bomb::ExplosionBundle;
use crate::GRID_SIZE;

fn spawn_explosion_at(
    mut commands: EntityCommands,
    handle: Handle<TextureAtlas>,
    transform: Transform,
    direction: ExplosionDirection,
) {
    use ExplosionDirection::*;
    commands
        .insert_bundle(ExplosionBundle::new(
            handle,
            transform,
            [direction, None, None, None],
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(6.0, 6.0))
        .insert(Sensor(true));
}

fn translate(transform: &Transform, inc_x: f32, inc_y: f32) -> Transform {
    transform.with_translation(Vec3::new(
        transform.translation.x + inc_x,
        transform.translation.y + inc_y,
        0.0,
    ))
}

pub fn explosion_expand(
    mut commands: Commands,
    atlas: Res<Atlas>,
    mut explosion_query: Query<(&mut Explosion, &Transform)>,
) {
    for (mut explosion, transform) in explosion_query.iter_mut() {
        if explosion.directions == FINISHED_EXPLOSION {
            continue;
        }

        for direction in explosion.directions.iter() {
            if *direction != ExplosionDirection::None {
                info!("Expanding bomb towards {:?}", direction);
            }
            match direction {
                ExplosionDirection::Up => {
                    spawn_explosion_at(
                        commands.spawn(),
                        atlas.handle.clone(),
                        translate(transform, 0.0, GRID_SIZE),
                        *direction,
                    );
                }
                ExplosionDirection::Down => {
                    spawn_explosion_at(
                        commands.spawn(),
                        atlas.handle.clone(),
                        translate(transform, 0.0, -GRID_SIZE),
                        *direction,
                    );
                }
                ExplosionDirection::Left => {
                    spawn_explosion_at(
                        commands.spawn(),
                        atlas.handle.clone(),
                        translate(transform, -GRID_SIZE, 0.0),
                        *direction,
                    );
                }
                ExplosionDirection::Right => {
                    spawn_explosion_at(
                        commands.spawn(),
                        atlas.handle.clone(),
                        translate(transform, GRID_SIZE, 0.0),
                        *direction,
                    );
                }
                _ => {}
            }
        }

        explosion.directions = FINISHED_EXPLOSION
    }
}
