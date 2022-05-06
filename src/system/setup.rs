use bevy::prelude::*;
use bevy_rapier2d::{plugin::RapierConfiguration, prelude::{RigidBody, Velocity, Collider, LockedAxes}};
use leafwing_input_manager::InputManagerBundle;

use crate::models::player::{Player, PlayerBundle};
use crate::models::bomb::{BombBundle};

#[derive(Component)]
struct Wall;

const E: usize = 0; // Edge
const G: usize = 7; // Grass
const S: usize = 4; // Shaded grass
const W: usize = 8; // Wall

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let texture_handle = asset_server.load("sprites/stage.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 3, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    rapier_config.gravity = Vec2::ZERO;
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let level = [
        [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
        [E, S, S, S, S, S, S, S, S, S, S, S, S, S, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, G, G, G, G, G, G, G, G, G, G, G, G, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, G, G, G, G, G, G, G, G, G, G, G, G, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, G, G, G, G, G, G, G, G, G, G, G, G, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, G, G, G, G, G, G, G, G, G, G, G, G, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, G, G, G, G, G, G, G, G, G, G, G, G, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, G, G, G, G, G, G, G, G, G, G, G, G, E],
        [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
    ];

    for (row_index, row) in level.iter().copied().rev().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            let wall_position = Vec2::new(column_index as f32 * (16.0), row_index as f32 * (16.0));

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
                })
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(16.0, 16.0));
        }
    }

    commands.spawn().insert_bundle(BombBundle::new(texture_atlas_handle.clone()));

    spawn_player(commands, texture_atlas_handle)
}

fn spawn_player(mut commands: Commands, atlas: Handle<TextureAtlas>) {
    let player_sprite = SpriteSheetBundle {
        texture_atlas: atlas,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform {
            ..default()
        },
        ..default()
    };
    
    commands.spawn_bundle(PlayerBundle {
        player: Player::One,
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::input_map(Player::One, None),
            ..default()
        },
        sprite: player_sprite,
    })
    .insert(RigidBody::Fixed)
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Velocity::zero())
    .insert(Collider::cuboid(16.0, 16.0));
}
