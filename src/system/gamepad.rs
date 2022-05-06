use bevy::input::gamepad::GamepadButton;
use bevy::prelude::*;

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
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::North)) {
            info!("{:?} just pressed North", gamepad);
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::North)) {
            info!("{:?} just released North", gamepad);
        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::East)) {
            info!("{:?} just pressed East", gamepad);
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::East)) {
            info!("{:?} just released East", gamepad);
        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::West)) {
            info!("{:?} just pressed West", gamepad);
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::West)) {
            info!("{:?} just released West", gamepad);
        }

        // Button C
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::C)) {
            info!("{:?} just pressed C", gamepad);
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::C)) {
            info!("{:?} just released C", gamepad);
        }
        // Button Z
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::Z)) {
            info!("{:?} just pressed Z", gamepad);
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::Z)) {
            info!("{:?} just released Z", gamepad);
        }

        // Dpad
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::DPadUp)) {
            info!("{:?} just pressed DPadUp", gamepad);
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::DPadUp)) {
            info!("{:?} just released DPadUp", gamepad);
        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::DPadDown)) {
            info!("{:?} just pressed DPadDown", gamepad);
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::DPadDown)) {
            info!("{:?} just released DPadDown", gamepad);
        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::DPadLeft)) {
            info!("{:?} just pressed DPadLeft", gamepad);
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::DPadLeft)) {
            info!("{:?} just released DPadLeft", gamepad);
        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::DPadRight)) {
            info!("{:?} just pressed DPadRight", gamepad);
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::DPadRight))
        {
            info!("{:?} just released DPadRight", gamepad);
        }

        // Left trigger
        let left_trigger1 = button_axes
            .get(GamepadButton(gamepad, GamepadButtonType::LeftTrigger))
            .unwrap();
        if left_trigger1.abs() > 0.01 {
            info!("{:?} LeftTrigger1 value is {}", gamepad, left_trigger1);
        }
        let left_trigger2 = button_axes
            .get(GamepadButton(gamepad, GamepadButtonType::LeftTrigger2))
            .unwrap();
        if left_trigger2.abs() > 0.01 {
            info!("{:?} LeftTrigger2 value is {}", gamepad, left_trigger2);
        }
        // Right trigger
        let right_trigger1 = button_axes
            .get(GamepadButton(gamepad, GamepadButtonType::RightTrigger))
            .unwrap();
        if right_trigger1.abs() > 0.01 {
            info!("{:?} RightTrigger1 value is {}", gamepad, right_trigger1);
        }
        let right_trigger2 = button_axes
            .get(GamepadButton(gamepad, GamepadButtonType::RightTrigger2))
            .unwrap();
        if right_trigger2.abs() > 0.01 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger2);
        }

        // Left stick
        let left_stick_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
        let left_stick_y = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > 0.01 {
            info!("{:?} LeftStickY value is {}", gamepad, left_stick_y);
        }

        // Right stick
        let right_stick_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::RightStickX))
            .unwrap();
        if right_stick_x.abs() > 0.01 {
            info!("{:?} RightStickX value is {}", gamepad, right_stick_x);
        }
        let right_stick_y = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::RightStickY))
            .unwrap();
        if right_stick_y.abs() > 0.01 {
            info!("{:?} RightStickY value is {}", gamepad, right_stick_y);
        }
    }
}
