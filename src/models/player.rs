use std::slice::Iter;

use bevy::{
    prelude::{Bundle, Component, Gamepad, GamepadButtonType, KeyCode},
    sprite::SpriteSheetBundle,
};
use leafwing_input_manager::{prelude::InputMap, Actionlike, InputManagerBundle};

pub const SPEED: f32 = 100.0;

#[derive(Component, Clone)]
pub enum Player {
    // Maps the player to a gamepad
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
}

impl Player {
    pub fn iter() -> Iter<'static, Player> {
        use Player::*;

        static PLAYERS: [Player; 4] = [One, Two, Three, Four];
        PLAYERS.iter()
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    #[bundle]
    pub input_manager: InputManagerBundle<Action>,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

#[derive(Actionlike, Clone)]
pub enum Action {
    Up,
    Right,
    Down,
    Left,
    PlaceBomb,
}

impl PlayerBundle {
    pub fn input_map(player: Player, gamepad: Option<Gamepad>) -> InputMap<Action> {
        let mut input_map = match player {
            Player::One => InputMap::new([
                (Action::Left, KeyCode::A),
                (Action::Right, KeyCode::D),
                (Action::Up, KeyCode::W),
                (Action::Down, KeyCode::S),
                (Action::PlaceBomb, KeyCode::Space),
            ]),

            Player::Two => InputMap::new([
                (Action::Left, KeyCode::Left),
                (Action::Right, KeyCode::Right),
                (Action::Up, KeyCode::Up),
                (Action::Down, KeyCode::Down),
                (Action::PlaceBomb, KeyCode::M),
            ]),
            _ => panic!("Unsupported"),
        };

        // Each player will use the same gamepad controls, but on seperate gamepads
        input_map
            .set_gamepad(gamepad.unwrap_or(Gamepad(player as usize)))
            .insert_multiple([
                (Action::Left, GamepadButtonType::DPadLeft),
                (Action::Right, GamepadButtonType::DPadRight),
                (Action::Up, GamepadButtonType::DPadUp),
                (Action::Down, GamepadButtonType::DPadDown),
                (Action::PlaceBomb, GamepadButtonType::South),
            ]);

        input_map
    }
}
