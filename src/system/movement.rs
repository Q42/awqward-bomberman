use crate::{
    models::player::Action,
    models::player::{self, Player},
    TIME_STEP,
};

use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub fn move_player_system(mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>) {
    let (action_state, mut transform) = query.single_mut();

    let mut delta = Vec2::new(0.0, 0.0);
    if action_state.pressed(Action::Up) {
        delta.y += 1.0;
    }
    if action_state.pressed(Action::Down) {
        delta.y -= 1.0;
    }

    if action_state.pressed(Action::Left) {
        delta.x -= 1.0;
    }
    if action_state.pressed(Action::Right) {
        delta.x += 1.0;
    }

    transform.translation.x += delta.x * TIME_STEP * player::SPEED;
    transform.translation.y += delta.y * TIME_STEP * player::SPEED;
}
