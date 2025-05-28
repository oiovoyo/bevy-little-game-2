//! # EchoNet Game
//!
//! This is the main entry point for the EchoNet game.
//! It sets up the Bevy application, registers plugins, states, resources, and components.

use bevy::prelude::*;

// Declare modules that will be in src/
mod components;
mod game_state;
mod gameplay_plugin; // This will look for src/gameplay_plugin.rs or src/gameplay_plugin/mod.rs
mod menu_plugin;
mod resources;
mod ui_plugin;

use crate::game_state::GameState;
use crate::menu_plugin::MenuPlugin;
use crate::gameplay_plugin::GameplayPlugin;
use crate::ui_plugin::UIPlugin; // CORRECTED: Capitalization
// CORRECTED: Removed LevelManager, GameTimer as they don't exist in resources.rs
use crate::resources::{CurrentLevel, PuzzleSpec, PlayerAttempt}; // GameFont is typically added by a plugin after loading

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "EchoNet".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        // Add custom resources (ensure these are defined in resources.rs and imported)
        .init_resource::<CurrentLevel>() 
        .init_resource::<PlayerAttempt>()
        .init_resource::<PuzzleSpec>() 
        // CORRECTED: Removed .init_resource for LevelManager and GameTimer
        .add_plugins((
            MenuPlugin,
            GameplayPlugin,
            UIPlugin, // CORRECTED: Matches corrected import
        ))
        .run();
}
