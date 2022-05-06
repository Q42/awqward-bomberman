use bevy::{input::Axis, input::Input, prelude::*};

use crate::models::player::Player;

struct Delta {
    x: f32,
    y: f32,
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut delta = Delta { x: 0.0, y: 0.0 };

    if keyboard_input.pressed(KeyCode::A) {
        delta.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        delta.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        delta.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        delta.y -= 1.0;
    }

    move_player(query.single_mut(), delta);
}

pub fn gamepad_system(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for gamepad in gamepads.iter().cloned() {
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::South)) {
            info!("{:?} just pressed South", gamepad);
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::South)) {
            info!("{:?} just released South", gamepad);
        }

        let right_trigger = button_axes
            .get(GamepadButton(gamepad, GamepadButtonType::RightTrigger2))
            .unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
    }
}

fn move_player(mut transform: Mut<'_, Transform>, delta: Delta) {
    transform.translation += Vec3::new(delta.x, delta.y, 0.0);
}
