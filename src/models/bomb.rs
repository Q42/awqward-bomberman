use bevy::{prelude::*, sprite::Anchor};

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
    pub fn new(atlas: Handle<TextureAtlas>) -> BombBundle {
        BombBundle {
            bomb: Bomb {
                remaining_time: INIT_REMAINING_TIME,
            },
            sprite_sheet: {
                SpriteSheetBundle {
                    texture_atlas: atlas,
                    sprite: TextureAtlasSprite {
                        index: 0,
                        anchor: Anchor::Center,
                        ..default()
                    },
                    ..default()
                }
            },
        }
    }
}
