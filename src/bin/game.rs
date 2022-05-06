use awqward_bomberman::{
    models::player::Action,
    system::{
        movement::move_player_system,
        scene::*,
        setup::*,
        explode_bomb::*
    },
    TIME_STEP,
};

use bevy::{core::FixedTimestep, prelude::*, DefaultPlugins};
use bevy_rapier2d::prelude::*;
use color_eyre::Report;
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::input::InputPlugin)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(save_scene_system.exclusive_system())
        .add_startup_system(load_scene_system)
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_player_system)
                .with_system(explode_bomb)
        )
        .run();

    Ok(())
}
