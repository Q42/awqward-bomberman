use bevy::prelude::*;

use crate::{model::player::Player, LAYER_TEXT};

#[derive(Component)]
struct FinishText;

pub fn end_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(Entity, &Player), With<Player>>,
) {
    if player_query.iter().count() != 1 {
        return;
    }

    let (entity, winner) = player_query.iter().next().unwrap();
    let font = asset_server.load("fonts/Roboto-Regular.ttf");
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                match winner {
                    Player::One => "Player 1 wins!",
                    Player::Two => "Player 2 wins!",
                    _ => "Draw!",
                },
                TextStyle {
                    font,
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, LAYER_TEXT)),
            ..default()
        })
        .insert(FinishText);
    commands.entity(entity).despawn();
}
