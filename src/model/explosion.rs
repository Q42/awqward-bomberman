use bevy::prelude::Component;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ExplosionDirection {
    None,
    Up,
    Right,
    Down,
    Left,
}

#[derive(Component)]
pub struct Explosion {
    pub timer: f32,
    pub directions: [ExplosionDirection; 4],
}

pub const FINISHED_EXPLOSION: [ExplosionDirection; 4] = {
    use ExplosionDirection::*;
    [None, None, None, None]
};

pub const NEW_EXPLOSION: [ExplosionDirection; 4] = {
    use ExplosionDirection::*;
    [Up, Down, Left, Right]
};
