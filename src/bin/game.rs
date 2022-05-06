use awqward_bomberman::{
    models::player::Action,
    system::{explode_bomb::*, movement::move_player_system, scene::*, setup::*},
    TIME_STEP,
};

use bevy::{core::FixedTimestep, prelude::*, DefaultPlugins};
use bevy_rapier2d::prelude::*;
use color_eyre::Report;
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bomberman".to_string(),
            width: awqward_bomberman::WINDOW_WIDTH,
            height: awqward_bomberman::WINDOW_HEIGHT,
            scale_factor_override: Some(4.0),
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::input::InputPlugin)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            awqward_bomberman::GRID_SIZE,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(save_scene_system.exclusive_system())
        .add_startup_system(load_scene_system)
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_player_system)
                .with_system(explode_bomb),
        )
        .run();

    Ok(())
}
