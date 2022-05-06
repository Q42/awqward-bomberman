use awqward_bomberman::{
    models::player::Action,
    system::{scene::*, setup, collision::{CollisionEvent, check_for_collisions}},
    TIME_STEP,
};

use bevy::{core::FixedTimestep, prelude::*, DefaultPlugins};
use color_eyre::Report;
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::input::InputPlugin)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(save_scene_system.exclusive_system())
        .add_startup_system(load_scene_system)
        .add_startup_system(setup::setup)
        .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)))
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
        )
        .run();

    Ok(())
}
