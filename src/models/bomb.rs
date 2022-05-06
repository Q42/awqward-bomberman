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
    pub fn new(atlas: Handle<TextureAtlas>, transform: Transform) -> BombBundle {
        BombBundle {
            bomb: Bomb {
                remaining_time: INIT_REMAINING_TIME,
            },
            sprite_sheet: {
                SpriteSheetBundle {
                    texture_atlas: atlas,
                    sprite: TextureAtlasSprite {
                        index: 4,
                        anchor: Anchor::Center,
                        ..default()
                    },
                    transform: round_transform_to_nearest_multiple_of(16.0, transform),
                    ..default()
                }
            },
        }
    }
}

fn round_transform_to_nearest_multiple_of(multiple: f32, transform: Transform) -> Transform {
    let x = ((transform.translation.x - 8.0) / multiple).ceil() * multiple;
    let y = ((transform.translation.y - 8.0) / multiple).ceil() * multiple;
    Transform::from_translation(Vec3::new(x, y, 0.0))
}