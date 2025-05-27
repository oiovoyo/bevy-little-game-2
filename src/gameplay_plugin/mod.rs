//! # Gameplay Plugin
//!
//! Orchestrates the main gameplay loop, including level setup,
//! node interactions, echo propagation, and puzzle validation.

use bevy::prelude::*;
use crate::game_state::GameState;
use crate::resources::{LevelManager, GameTimer, PlayerActivationSequence, EchoPath};
use crate::components::{Node, NodeState, Connection, DataEcho, ClickableNode, PuzzleElement};

// Declare gameplay submodules
pub mod node;
pub mod connection;
pub mod puzzle;
pub mod echo;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources specific to gameplay that might need resetting
            .init_resource::<PlayerActivationSequence>()
            .init_resource::<EchoPath>()
            // Systems to run when entering the LevelLoading state
            .add_systems(OnEnter(GameState::LevelLoading), setup_level)
            // Systems to run during Gameplay state
            .add_systems(Update,
                (
                    node::node_interaction_system, // Handles player clicks on nodes
                    node::update_node_visuals_system, // Updates visuals based on NodeState
                    connection::draw_connections_system, // Draws connections between nodes
                    echo::spawn_echo_system, // Spawns echos when start nodes are ready
                    echo::update_echo_movement_system, // Moves echos along paths
                    echo::despawn_echo_at_target_system, // Despawns echos when they reach their target
                    puzzle::check_level_completion_system, // Checks if level objectives are met
                    puzzle::check_level_fail_system, // Checks for failure conditions (e.g. timer)
                    game_timer_system, // Updates the game timer
                )
                .run_if(in_state(GameState::Gameplay)),
            )
            // Systems to run when exiting the Gameplay state (cleanup)
            .add_systems(OnExit(GameState::Gameplay), cleanup_gameplay_elements);
    }
}

/// Sets up the current level based on LevelManager.
/// This includes spawning nodes, connections, and setting up the timer.
fn setup_level(
    mut commands: Commands,
    mut level_manager: ResMut<LevelManager>,
    mut game_timer: ResMut<GameTimer>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_activation_sequence: ResMut<PlayerActivationSequence>,
    mut echo_path: ResMut<EchoPath>,
    // asset_server: Res<AssetServer>, // For loading sprites/meshes if any. Removed as not used in current version.
) {
    // Clear previous level's data
    player_activation_sequence.clear();
    echo_path.clear();

    let level = match level_manager.get_current_level() {
        Some(l) => l.clone(), // Clone to avoid borrowing issues if we modify LevelManager later
        None => {
            if level_manager.current_level_index >= level_manager.levels.len() {
                 game_state.set(GameState::GameWon);
            } else {
                 game_state.set(GameState::MainMenu);
            }
            return;
        }
    };

    // Setup GameTimer
    if let Some(time_limit) = level.time_limit {
        game_timer.timer = Timer::from_seconds(time_limit, TimerMode::Once);
        game_timer.time_limit_seconds = Some(time_limit);
    } else {
        game_timer.timer = Timer::from_seconds(0.0, TimerMode::Once); // Effectively no timer
        game_timer.time_limit_seconds = None;
    }
    game_timer.timer.reset(); // Ensure timer starts fresh

    // Spawn nodes
    let mut node_entities: Vec<Entity> = Vec::new();
    for (i, (pos, initial_state, puzzle_info)) in level.nodes.iter().enumerate() {
        let mut node_entity_commands = commands.spawn((
            Node { id: i }, 
            *initial_state,
            SpriteBundle { 
                sprite: Sprite {
                    color: color_for_node_state(initial_state),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },
            ClickableNode, 
            Name::new(format!("Node {}", i)),
        ));
        if let Some(puzzle_element) = puzzle_info {
            node_entity_commands.insert(*puzzle_element);
        }
        node_entities.push(node_entity_commands.id());
    }

    // Spawn connections
    for (idx_a, idx_b) in level.connections.iter() {
        if let (Some(&entity_a), Some(&entity_b)) = (node_entities.get(*idx_a), node_entities.get(*idx_b)) {
            commands.spawn((
                Connection {
                    node_a: entity_a,
                    node_b: entity_b,
                    is_active: false,
                },
                Name::new(format!("Connection {}-{}", idx_a, idx_b)),
            ));
        }
    }
    
    game_state.set(GameState::Gameplay);
}


/// Cleans up all gameplay elements (nodes, connections, echos) when exiting gameplay.
fn cleanup_gameplay_elements(
    mut commands: Commands,
    node_query: Query<Entity, With<Node>>,
    connection_query: Query<Entity, With<Connection>>,
    echo_query: Query<Entity, With<DataEcho>>,
) {
    for entity in node_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in connection_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in echo_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// System to update the game timer.
fn game_timer_system(
    time: Res<Time>,
    mut game_timer: ResMut<GameTimer>,
) {
    if game_timer.time_limit_seconds.is_some() {
        game_timer.timer.tick(time.delta());
    }
}

// Helper function for node color, might be moved to node.rs or a visuals module
// It's pub because it's used by node.rs which is now a submodule.
pub fn color_for_node_state(state: &NodeState) -> Color {
    match state {
        NodeState::Idle => Color::rgb(0.5, 0.5, 0.5),       // Grey
        NodeState::Activating => Color::YELLOW,             // Yellow
        NodeState::Active => Color::GREEN,                  // Green
        NodeState::Target => Color::BLUE,                   // Blue
        NodeState::Start => Color::ORANGE_RED,              // OrangeRed
    }
}
File created successfully.
