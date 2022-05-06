use crate::models::atlas::Atlas;
use crate::models::bomb::Bomb;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::prelude::{RigidBody, Collider, Sensor};

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionCore;

#[derive(Bundle)]
pub struct ExplosionBundle {
    pub explosion: Explosion,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

impl ExplosionBundle {
    pub fn new(atlas: Handle<TextureAtlas>, transform: Transform) -> ExplosionBundle {
        ExplosionBundle {
            explosion: Explosion,
            sprite_sheet: {
                SpriteSheetBundle {
                    texture_atlas: atlas,
                    sprite: TextureAtlasSprite {
                        index: 18,
                        anchor: Anchor::Center,
                        ..default()
                    },
                    transform,
                    ..default()
                }
            },
        }
    }
}

pub fn explode_bomb(
    mut commands: Commands,
    mut bomb_query: Query<(Entity, &mut Bomb, &Transform)>,
    atlas: Res<Atlas>,
    time: Res<Time>,
) {
    for (entity, mut bomb, transform) in bomb_query.iter_mut() {
        bomb.remaining_time -= time.delta().as_secs_f32();

        if bomb.remaining_time <= 0.0 {
            commands.spawn()
                .insert_bundle(ExplosionBundle::new(atlas.handle.clone(), *transform))
                .insert(ExplosionCore)
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(16.0, 16.0))
                .insert(Sensor(true));
            commands.entity(entity).despawn();
        }
    }
}