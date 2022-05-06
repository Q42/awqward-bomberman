use awqward_bomberman::{
    models::player::Action,
    models::player::{self, Player},
    system::{
        collision::{check_for_collisions, CollisionEvent},
        scene::*,
        setup,
    },
    TIME_STEP,
};

use bevy::{core::FixedTimestep, prelude::*, DefaultPlugins};
use color_eyre::Report;
use leafwing_input_manager::{plugin::InputManagerPlugin, prelude::ActionState};

fn move_player_system(mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>) {
    let (action_state, mut transform) = query.single_mut();

    let mut delta = Vec2::new(0.0, 0.0);
    if action_state.pressed(Action::Up) {
        info!("pressed up");
        delta.y += 1.0;
    }
    if action_state.pressed(Action::Down) {
        delta.y -= 1.0;
    }

    if action_state.pressed(Action::Left) {
        delta.x -= 1.0;
    }
    if action_state.pressed(Action::Right) {
        delta.x += 1.0;
    }

    transform.translation.x += delta.x * TIME_STEP * player::SPEED;
    transform.translation.y += delta.y * TIME_STEP * player::SPEED;
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::input::InputPlugin)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(save_scene_system.exclusive_system())
        .add_startup_system(load_scene_system)
        .add_startup_system(setup::setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(move_player_system),
        )
        .run();

    Ok(())
}
