use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::{
    plugin::RapierConfiguration,
    prelude::{Collider, Friction, LockedAxes, RigidBody, Velocity},
};
use leafwing_input_manager::InputManagerBundle;

use crate::models::player::{Player, PlayerBundle};
use crate::{models::bomb::BombBundle, GRID_SIZE};
use crate::models::atlas::{Atlas};

#[derive(Component)]
struct Wall;

const E: usize = 0; // Edge
const W: usize = 1; // Wall
const S: usize = 2; // Shaded grass
const G: usize = 3; // Grass

const BOMB: usize = 4;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("sprites/atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 5, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(Atlas { handle: texture_atlas_handle.clone() });

    let level = [
        [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
        [E, S, S, S, S, S, S, S, S, S, S, S, S, S, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, S, G, S, G, S, G, S, G, S, G, S, G, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, S, G, S, G, S, G, S, G, S, G, S, G, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, S, G, S, G, S, G, S, G, S, G, S, G, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, S, G, S, G, S, G, S, G, S, G, S, G, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, S, G, S, G, S, G, S, G, S, G, S, G, E],
        [E, G, W, G, W, G, W, G, W, G, W, G, W, G, E],
        [E, G, S, G, S, G, S, G, S, G, S, G, S, G, E],
        [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
    ];

    for (row_index, row) in level.iter().copied().rev().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            let wall_position = Vec2::new(
                ((column_index) as f32 * (GRID_SIZE) + 8.0) - (crate::WINDOW_WIDTH / 2.0),
                ((row_index) as f32 * (GRID_SIZE) + 8.0) - (crate::WINDOW_HEIGHT / 2.0),
            );

            let mut environment = commands.spawn();

            environment.insert(Wall).insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: *column,
                    anchor: Anchor::Center,
                    ..default()
                },
                transform: Transform {
                    translation: wall_position.extend(0.0),
                    ..default()
                },
                ..default()
            });

            if *column != G && *column != S {
                environment
                    .insert(RigidBody::Fixed)
                    .insert(Collider::cuboid(8.0, 8.0));
            }
        }
    }

    spawn_player(commands, texture_atlas_handle)
}

fn spawn_player(mut commands: Commands, atlas: Handle<TextureAtlas>) {
    let player_sprite = SpriteSheetBundle {
        texture_atlas: atlas,
        sprite: TextureAtlasSprite::new(5),
        transform: Transform { ..default() },
        ..default()
    };

    commands
        .spawn_bundle(PlayerBundle {
            player: Player::One,
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::input_map(Player::One, None),
                ..default()
            },
            sprite: player_sprite,
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        .insert(Friction::coefficient(0.0))
        .insert(Collider::ball(8.0));
}