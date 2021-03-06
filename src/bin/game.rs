use awqward_bomberman::{
    model::player::Action,
    system::{
        end_game, explode_bomb::*, explosion_clear::*, explosion_expand::*, explosion_kill::*,
        explosion_stop_expand::*, movement, place_bomb, setup::*,
    },
    TIME_STEP,
};

use bevy::{core::FixedTimestep, prelude::*, DefaultPlugins};
use bevy_rapier2d::prelude::*;
use color_eyre::Report;
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() -> Result<(), Report> {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    color_eyre::install()?;

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Awqward bomberman".to_string(),
            width: awqward_bomberman::WINDOW_WIDTH,
            height: awqward_bomberman::WINDOW_HEIGHT,
            scale_factor_override: Some(4.0),
            resizable: cfg!(debug_assertions),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            awqward_bomberman::GRID_SIZE,
        ))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(movement::move_player_system)
                .with_system(place_bomb::place_bomb)
                .with_system(end_game::end_game),
        )
        .add_system_set(
            SystemSet::new()
                .with_system(explode_bomb)
                .with_system(explosion_clear.after(explode_bomb))
                .with_system(explosion_stop_expand.after(explosion_clear))
                .with_system(explosion_expand.after(explosion_stop_expand))
                .with_system(explosion_kill.after(explosion_expand)),
        )
        .run();

    Ok(())
}
