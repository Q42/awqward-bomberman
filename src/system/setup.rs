use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::InputManagerBundle;

use crate::model::player::{Player, PlayerBundle};
use crate::model::{atlas::Atlas, destructable::Destructable};
use crate::{D, E, G, GRID_SIZE, LAYER_BACKGROUND, LAYER_PLAYER, LAYER_WALLS, PLAYER, S, W};

#[derive(Component)]
pub struct Wall(pub usize);

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
        [E, S, D, S, D, D, D, S, D, D, S, D, S, S, E],
        [E, D, W, D, W, D, W, D, W, G, W, D, W, G, E],
        [E, D, S, G, S, D, S, D, D, D, D, D, S, D, E],
        [E, D, W, D, W, G, W, D, W, D, W, G, W, G, E],
        [E, G, D, D, D, G, D, D, S, G, D, D, D, D, E],
        [E, D, W, D, W, G, W, D, W, D, W, D, W, G, E],
        [E, G, D, D, S, D, D, G, D, D, S, G, S, D, E],
        [E, D, W, G, W, D, W, G, W, D, W, D, W, D, E],
        [E, G, D, D, S, G, S, D, D, G, S, D, S, D, E],
        [E, D, W, D, W, D, W, G, W, D, W, G, W, G, E],
        [E, D, D, G, D, G, S, D, D, G, S, D, S, D, E],
        [E, G, W, D, W, G, W, G, W, G, W, G, W, D, E],
        [E, G, S, D, D, G, D, G, S, D, D, G, D, G, E],
        [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
    ];

    for (row_index, row) in level.iter().copied().rev().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            let environment_position = Vec3::new(
                ((column_index) as f32 * (GRID_SIZE) + 8.0) - (crate::WINDOW_WIDTH / 2.0),
                ((row_index) as f32 * (GRID_SIZE) + 8.0) - (crate::WINDOW_HEIGHT / 2.0),
                if *column != G && *column != S && *column != D {
                    LAYER_WALLS
                } else {
                    LAYER_BACKGROUND
                },
            );

            let mut entity = commands.spawn();

            entity.insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: *column,
                    anchor: Anchor::Center,
                    ..default()
                },
                transform: Transform {
                    translation: environment_position,
                    ..default()
                },
                ..default()
            });

            if *column == D {
                entity.insert(Destructable);
            }

            if *column != G && *column != S {
                entity
                    .insert(Wall(*column))
                    .insert(RigidBody::Fixed)
                    .insert(Collider::cuboid(8.0, 8.0));
            }
        }
    }

    spawn_player(
        commands.spawn(),
        texture_atlas_handle.clone(),
        Player::One,
        Vec2::new(
            (GRID_SIZE * 2.0) - (crate::WINDOW_WIDTH / 2.0) - 8.0,
            (GRID_SIZE * 2.0) - (crate::WINDOW_HEIGHT / 2.0) - 8.0,
        ),
    );
    spawn_player(
        commands.spawn(),
        texture_atlas_handle,
        Player::Two,
        Vec2::new(
            (crate::WINDOW_WIDTH / 2.0) - GRID_SIZE - 8.0,
            (crate::WINDOW_HEIGHT / 2.0) - GRID_SIZE - 8.0,
        ),
    );
}

fn spawn_player(
    mut commands: EntityCommands,
    atlas: Handle<TextureAtlas>,
    player: Player,
    position: Vec2,
) {
    let sprite = SpriteSheetBundle {
        texture_atlas: atlas,
        sprite: TextureAtlasSprite::new(PLAYER),
        transform: Transform {
            translation: position.extend(LAYER_PLAYER),
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
            sprite,
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        .insert(Friction::coefficient(0.0))
        .insert(Collider::ball(8.0))
        .insert(ActiveEvents::COLLISION_EVENTS);
}
