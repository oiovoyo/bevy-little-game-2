//! # Game State Module
//!
//! Defines the different states the game can be in.
//! This is crucial for controlling game flow, such as showing menus,
//! managing gameplay loops, or displaying game over/completion screens.

use bevy::prelude::*;

/// Enum representing the possible states of the game.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,       // The game is showing the main menu.
    LevelLoading,   // The game is loading assets/setting up for the next level.
    Gameplay,       // The player is actively playing a level.
    LevelComplete,  // The player has successfully completed a level.
    GameOver,       // The player has failed a level (e.g., time ran out).
    GameWon,        // The player has completed all levels.
}
File created successfully.
