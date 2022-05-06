use bevy::log::info;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::models::atlas::Atlas;
use crate::models::bomb::BombBundle;
use crate::models::player::{Action, Player};

pub fn place_bomb(
    mut commands: Commands,
    atlas: Res<Atlas>,
    mut query: Query<(&ActionState<Action>, &Transform), With<Player>>,
) {
    for (action_state, transform) in query.iter_mut() {
        if action_state.just_pressed(Action::PlaceBomb) {
            info!("Place bomb");
            commands
                .spawn()
                .insert_bundle(BombBundle::new(atlas.handle.clone(), *transform));
        }
    }
}
