//! # EchoNet Game
//!
//! This is the main entry point for the EchoNet game.
//! It sets up the Bevy application, registers plugins, states, resources, and components.

use bevy::prelude::*;

// Declare modules that will be in src/
mod components;
mod game_state;
mod gameplay_plugin;
mod menu_plugin;
mod resources;
mod ui_plugin;

fn main() {
    App::new()
        // Add Bevy's core plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "EchoNet".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        // Initialize game states
        .init_state::<game_state::GameState>()
        // Add custom resources
        .init_resource::<resources::LevelManager>()
        .init_resource::<resources::GameTimer>()
        // Add custom plugins
        .add_plugins((
            menu_plugin::MenuPlugin,
            gameplay_plugin::GameplayPlugin,
            ui_plugin::UiPlugin,
        ))
        // Add a simple camera setup system
        .add_systems(Startup, setup_camera)
        .run();
}

/// Sets up a 2D camera for the game.
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
File created successfully.
