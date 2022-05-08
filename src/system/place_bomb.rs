use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::model::atlas::Atlas;
use crate::model::bomb::BombBundle;
use crate::model::player::{Action, Player};

pub fn place_bomb(
    mut commands: Commands,
    atlas: Res<Atlas>,
    mut query: Query<(&ActionState<Action>, &Transform), With<Player>>,
) {
    for (action_state, transform) in query.iter_mut() {
        if action_state.just_pressed(Action::PlaceBomb) {
            commands
                .spawn()
                .insert_bundle(BombBundle::new(atlas.handle.clone(), *transform));
        }
    }
}
