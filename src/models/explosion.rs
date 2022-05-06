use bevy::{prelude::*};

#[derive(Component)]
pub struct Explosion {
  pub remaining_time: f32
}

#[derive(Component)]
pub struct ExplosionCore;