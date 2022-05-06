use crate::{
    models::player::Action,
    models::player::{self, Player},
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::prelude::ActionState;

pub fn move_player_system(mut query: Query<(&ActionState<Action>, &mut Velocity), With<Player>>) {
    for (action_state, mut velocity) in query.iter_mut() {
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

        if delta != Vec2::ZERO {
            delta /= delta.length();
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        velocity.linvel = delta * player::SPEED;
    }
}
