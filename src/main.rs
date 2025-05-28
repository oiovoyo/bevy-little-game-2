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
use crate::ui_plugin::UiPlugin;
// The resources CurrentLevel, PuzzleSpec, PlayerAttempt, GameFont were not part of the
// previously established resources.rs. Reverting to LevelManager and GameTimer.
use crate::resources::{LevelManager, GameTimer}; 


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "EchoNet".into(),
                // Resolution changed from 1280x720 in original generation to 800x600 here.
                // Keeping 800x600 as per this specific instruction.
                resolution: (800.0, 600.0).into(), 
                ..default()
            }),
            ..default()
        }))
        // Initialize GameState
        .init_state::<GameState>() 
        // Add custom resources 
        .init_resource::<LevelManager>() 
        .init_resource::<GameTimer>()
        // GameFont is not init_resource'd as it's an asset.
        // It should be loaded and inserted as a resource by a relevant plugin (e.g., ui_plugin or menu_plugin).
        // Add custom plugins
        .add_plugins((
            MenuPlugin,
            GameplayPlugin,
            UiPlugin, // Changed from UIPlugin
        ))
        // setup_camera was present in the original generation but removed in the self-correction.
        // DefaultPlugins usually adds a camera if one isn't present, or specific plugins add their own.
        // For a minimal main, it's fine to omit it if plugins handle their camera needs.
        // .add_systems(Update, bevy::window::close_on_esc) // This is often default behavior.
        .run();
}
