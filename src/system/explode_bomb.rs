use crate::model::atlas::Atlas;
use crate::model::bomb::Bomb;
use crate::model::explosion::{Explosion, ExplosionDirection, NEW_EXPLOSION};
use crate::{EXPLOSION_CENTER, EXPLOSION_HORIZONTAL, EXPLOSION_VERTICAL};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct ExplosionBundle {
    pub explosion: Explosion,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

impl ExplosionBundle {
    pub fn new(
        atlas: Handle<TextureAtlas>,
        transform: Transform,
        directions: [ExplosionDirection; 4],
    ) -> ExplosionBundle {
        ExplosionBundle {
            explosion: Explosion {
                timer: 1.0,
                directions,
            },
            sprite_sheet: {
                SpriteSheetBundle {
                    texture_atlas: atlas,
                    sprite: TextureAtlasSprite {
                        index: explosion_sprite_index(directions),
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

fn explosion_sprite_index(directions: [ExplosionDirection; 4]) -> usize {
    use crate::model::explosion::ExplosionDirection::*;
    match directions {
        [Left, None, None, None] | [Right, None, None, None] => EXPLOSION_HORIZONTAL,
        [Up, None, None, None] | [Down, None, None, None] => EXPLOSION_VERTICAL,
        _ => EXPLOSION_CENTER
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
            commands.entity(entity).despawn();
            commands
                .spawn()
                .insert_bundle(ExplosionBundle::new(
                    atlas.handle.clone(),
                    *transform,
                    NEW_EXPLOSION,
                ))
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(6.0, 6.0))
                .insert(Sensor(true));
        }
    }
}
