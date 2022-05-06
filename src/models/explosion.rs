use bevy::prelude::Component;

pub enum ExplosionDirection {
    None,
    Up,
    Right,
    Down,
    Left,
}

#[derive(Component)]
pub struct Explosion(pub f32, pub [ExplosionDirection; 4]);
