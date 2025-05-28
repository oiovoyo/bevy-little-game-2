use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    LoadingLevel, // Intermediary state to setup levels
    Playing,
    LevelComplete,
}
