use bevy::prelude::*;
use bevy::log::info;
use leafwing_input_manager::prelude::ActionState;

use crate::models::bomb::{BombBundle};
use crate::models::atlas::{Atlas};
use crate::models::player::{Player, Action};

pub fn place_bomb(mut commands: Commands, atlas: Res<Atlas>, mut query: Query<(&ActionState<Action>, &Transform), With<Player>>) {
  let (action_state, transform) = query.single_mut();
  
  if action_state.just_pressed(Action::PlaceBomb) {
    info!("Place bomb");
    commands.spawn().insert_bundle(BombBundle::new(atlas.handle.clone(), *transform));
  }
}