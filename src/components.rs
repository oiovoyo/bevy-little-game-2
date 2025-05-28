use bevy::prelude::*;

#[derive(Component)]
pub struct Node {
    pub id: usize,
    pub original_color: Color,
}

#[derive(Component)]
pub struct ActivatedNode; // Marker component for currently activated node

#[derive(Component)]
pub struct Connection {
    pub start_node_entity: Entity,
    pub end_node_entity: Entity,
}

#[derive(Component)]
pub struct MainMenuUI; // Marker for main menu UI elements

#[derive(Component)]
pub struct GameplayUI; // Marker for gameplay UI elements

#[derive(Component)]
pub struct LevelCompleteUI; // Marker for level complete UI elements

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
pub enum GameButtonAction {
    NextLevel,
    RestartLevel,
    BackToMenu,
}
