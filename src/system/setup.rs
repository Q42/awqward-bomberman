use bevy::prelude::*;

use crate::models::player::Player;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/stage.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 3, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(8),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
    });

    // Player 1
    commands.spawn().insert(Player).insert_bundle(SpriteBundle {
        texture: asset_server.load("sprites/bomber_barbarian.png"),
        transform: Transform {
            scale: Vec3::new(0.1, 0.1, 1.0),
            ..default()
        },
        ..default()
    });
}
