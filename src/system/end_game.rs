use bevy::prelude::*;

use crate::models::player::Player;

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
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..default()
        })
        .insert(FinishText);
    commands.entity(entity).despawn();
}
