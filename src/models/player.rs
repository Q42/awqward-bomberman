use std::slice::Iter;

use bevy::{
    prelude::{Bundle, Component, Gamepad, GamepadButtonType, KeyCode},
    sprite::SpriteBundle,
};
use leafwing_input_manager::{prelude::InputMap, Actionlike, InputManagerBundle};

use crate::system::collision::Collider;

#[derive(Component)]
pub enum Player {
    // Maps the player to a gamepad
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
}

impl Player {
    pub fn iter() -> Iter<'static, Player> {
        static PLAYERS: [Player; 4] = [Player::One, Player::Two, Player::Three, Player::Four];
        PLAYERS.iter()
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    #[bundle]
    pub input_manager: InputManagerBundle<Action>,
    #[bundle]
    pub sprite: SpriteBundle,
    pub collider: Collider,
}

#[derive(Actionlike, Clone)]
pub enum Action {
    Up,
    Right,
    Down,
    Left,
}

impl PlayerBundle {
    pub fn input_map(player: Player) -> InputMap<Action> {
        let mut input_map = InputMap::new([
            (Action::Left, KeyCode::A),
            (Action::Right, KeyCode::D),
            (Action::Up, KeyCode::W),
            (Action::Down, KeyCode::S),
        ])
        .set_gamepad(Gamepad(player as usize))
        .build();

        // Each player will use the same gamepad controls, but on seperate gamepads
        input_map.insert_multiple([
            (Action::Left, GamepadButtonType::DPadLeft),
            (Action::Right, GamepadButtonType::DPadRight),
            (Action::Up, GamepadButtonType::DPadUp),
            (Action::Down, GamepadButtonType::DPadDown),
        ]);

        input_map
    }
}
