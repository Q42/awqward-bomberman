use bevy::{prelude::*, sprite::Anchor};
use crate::{BOMB, LAYER_ITEMS};

const INIT_REMAINING_TIME: f32 = 5.0;

#[derive(Component)]
pub struct Bomb {
    pub remaining_time: f32,
}

#[derive(Bundle)]
pub struct BombBundle {
    pub bomb: Bomb,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

impl BombBundle {
    pub fn new(atlas: Handle<TextureAtlas>, transform: Transform) -> BombBundle {
        BombBundle {
            bomb: Bomb {
                remaining_time: INIT_REMAINING_TIME,
            },
            sprite_sheet: {
                SpriteSheetBundle {
                    texture_atlas: atlas,
                    sprite: TextureAtlasSprite {
                        index: BOMB,
                        anchor: Anchor::Center,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(transform.translation.x, transform.translation.y, LAYER_ITEMS),
                        ..transform
                    },
                    ..default()
                }
            },
        }
    }
}
