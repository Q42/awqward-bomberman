use awqward_bomberman::system::scene::*;
use awqward_bomberman::system::setup::*;

use bevy::{prelude::*, DefaultPlugins};
use color_eyre::Report;

/// This example illustrates loading and saving scenes from files
fn main() -> Result<(), Report> {
    color_eyre::install()?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(save_scene_system.exclusive_system())
        .add_startup_system(load_scene_system)
        .add_startup_system(setup)
        .run();

    Ok(())
}
