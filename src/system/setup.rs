use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Anchor};
use bevy_rapier2d::{
    plugin::RapierConfiguration,
    prelude::{Collider, Friction, LockedAxes, RigidBody, Velocity},
};
use leafwing_input_manager::InputManagerBundle;

use crate::models::atlas::Atlas;
use crate::models::player::{Player, PlayerBundle};
use crate::{E, G, GRID_SIZE, LAYER_PLAYER, PLAYER, S, W};

#[derive(Component)]
struct Wall;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("sprites/atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 9, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(Atlas {
        handle: texture_atlas_handle.clone(),
    });

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

    spawn_player(commands.spawn(), texture_atlas_handle.clone(), Player::One);
    spawn_player(commands.spawn(), texture_atlas_handle, Player::Two);
}

fn spawn_player(mut commands: EntityCommands, atlas: Handle<TextureAtlas>, player: Player) {
    let player_sprite = SpriteSheetBundle {
        texture_atlas: atlas,
        sprite: TextureAtlasSprite::new(PLAYER),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, LAYER_PLAYER),
            ..default()
        },
        ..default()
    };

    commands
        .insert_bundle(PlayerBundle {
            player: player.clone(),
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::input_map(player, None),
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
