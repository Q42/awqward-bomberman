use bevy::{prelude::*};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(SpriteBundle {
      texture: asset_server.load("sprites/stage.png"),
      ..default()
  });
}