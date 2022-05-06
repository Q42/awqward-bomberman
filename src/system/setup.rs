use bevy::{prelude::*};

#[derive(Component)]
struct Wall;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
  let texture_handle = asset_server.load("sprites/stage.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 3, 3);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  let level = [
      [0, 0, 0, 0],
      [0, 7, 0, 0],
      [0, 0, 0, 0],
  ];

  for (row_index, row) in level.iter().enumerate() {
    for (column_index, column) in row.iter().enumerate() {
        let wall_position = Vec2::new(
            0.0 + column_index as f32 * (16.0),
            0.0 + row_index as f32 * (16.0),
        );

        // brick
        commands
          .spawn()
          .insert(Wall)
          .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(*column),
            transform: Transform {
              translation: wall_position.extend(0.0),
              ..default()
            },
            ..default()
          });
    }
}
}