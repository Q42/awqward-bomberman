use std::slice::Iter;

use bevy::{
    prelude::{Bundle, Component, Gamepad, GamepadButtonType, KeyCode},
    sprite::SpriteSheetBundle,
};
use leafwing_input_manager::{prelude::InputMap, Actionlike, InputManagerBundle};

pub const SPEED: f32 = 100.0;

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
    PlaceBomb
}

impl PlayerBundle {
    pub fn input_map(player: Player, gamepad: Option<Gamepad>) -> InputMap<Action> {
        let mut input_map = gamepad.map_or_else(
            || {
                InputMap::new([
                    (Action::Left, KeyCode::A),
                    (Action::Right, KeyCode::D),
                    (Action::Up, KeyCode::W),
                    (Action::Down, KeyCode::S),
                    (Action::PlaceBomb, KeyCode::Space)
                ])
                .set_gamepad(Gamepad(player as usize))
                .build()
            },
            |gamepad| InputMap::default().set_gamepad(gamepad).build(),
        );

        // Each player will use the same gamepad controls, but on seperate gamepads
        input_map.insert_multiple([
            (Action::Left, GamepadButtonType::DPadLeft),
            (Action::Right, GamepadButtonType::DPadRight),
            (Action::Up, GamepadButtonType::DPadUp),
            (Action::Down, GamepadButtonType::DPadDown),
            (Action::PlaceBomb, GamepadButtonType::South),
        ]);

        input_map
    }
}
