use bevy::{prelude::*};

pub enum ExplosionDirection {
    None,
    Up,
    Right,
    Down,
    Left,
}

#[derive(Component)]
pub struct Explosion(pub [ExplosionDirection; 4]);