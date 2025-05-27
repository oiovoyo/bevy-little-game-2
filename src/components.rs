//! # Components Module
//!
//! Defines the core components used in the EchoNet game.
//! These components are attached to entities to give them specific properties or behaviors.

use bevy::prelude::*;

/// Marker component for a generic network node.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: usize,
}

/// Component indicating the state of a node.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Idle,
    Activating, // Node is in the process of being activated
    Active,     // Node has been successfully activated
    Target,     // This node is a target for the echo
    Start,      // This node is a starting point for an echo
}

impl Default for NodeState {
    fn default() -> Self {
        NodeState::Idle
    }
}

/// Component representing a connection between two nodes.
/// Stores the entity IDs of the connected nodes.
#[derive(Component, Debug, Clone, Copy)]
pub struct Connection {
    pub node_a: Entity,
    pub node_b: Entity,
    pub is_active: bool, // True if the connection is part of an active path
}

/// Component representing a "Data Echo" that travels along connections.
#[derive(Component, Debug)]
pub struct DataEcho {
    pub current_node: Entity,
    pub path: Vec<Entity>, // Path the echo has taken
    pub target_node: Entity,
    pub speed: f32, // Speed at which the echo travels
    pub progress_on_connection: f32, // 0.0 to 1.0, progress along current connection segment
    pub current_segment_index: usize, // Index of the current node in its planned path
}

/// Marker component for UI elements specific to the main menu.
#[derive(Component)]
pub struct MainMenuUI;

/// Marker component for UI elements specific to the gameplay screen.
#[derive(Component)]
pub struct GameplayUI;

/// Marker component for UI elements specific to the level complete screen.
#[derive(Component)]
pub struct LevelCompleteUI;

/// Component for a button that starts the game or a specific level.
#[derive(Component)]
pub struct StartButton;

/// Component for a button that navigates to the next level.
#[derive(Component)]
pub struct NextLevelButton;

/// Component for a button that returns to the main menu.
#[derive(Component)]
pub struct MainMenuButton;

/// Component to display the current level number.
#[derive(Component)]
pub struct LevelDisplay;

/// Component to display instructions or game messages.
#[derive(Component)]
pub struct InfoTextDisplay;

/// Component to display a timer during gameplay.
#[derive(Component)]
pub struct TimerTextDisplay;

/// Component to associate a visual representation (e.g., sprite or mesh) with a node.
#[derive(Component)]
pub struct NodeVisual {
    // Potentially store color, shape, etc.
}

/// Component to associate a visual representation with a connection.
#[derive(Component)]
pub struct ConnectionVisual {
    // Potentially store line thickness, color, etc.
}

/// Component to associate a visual representation with an echo.
#[derive(Component)]
pub struct EchoVisual {
    // Potentially store color, particle effects, etc.
}

/// Component identifying an entity as part of the puzzle definition for a level.
/// This could be attached to nodes that need to be activated in a specific order.
#[derive(Component, Debug, Clone, Copy)]
pub struct PuzzleElement {
    pub required_order: Option<usize>, // If part of a sequence
    pub activation_time_limit: Option<f32>, // Optional time limit to activate this element
}

/// Component to mark a node as clickable.
/// Used for player interaction.
#[derive(Component)]
pub struct ClickableNode;
File created successfully.
