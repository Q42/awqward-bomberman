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
                        translation: round_transform_to_nearest_multiple_of(16.0, transform.translation),
                        ..transform
                    },
                    ..default()
                }
            },
        }
    }
}

fn round_transform_to_nearest_multiple_of(multiple: f32, translation: Vec3) -> Vec3 {
    let x = ((translation.x - 8.0) / multiple).ceil() * multiple;
    let y = ((translation.y - 8.0) / multiple).ceil() * multiple;
    Vec3::new(x, y, LAYER_ITEMS)
}