//! # Node Gameplay Logic

use bevy::prelude::*;
use crate::components::{Node, NodeState, ClickableNode, PuzzleElement};
use crate::resources::{PlayerActivationSequence, EchoPath, LevelManager};
// use crate::game_state::GameState; // Not directly used here, but could be for node-specific state changes
use super::color_for_node_state; // Accessing helper from parent module (gameplay_plugin/mod.rs)

/// System to handle player interaction with nodes (e.g., clicking).
pub fn node_interaction_system(
    mut interaction_query: Query<(&Interaction, &mut NodeState, &Node, Entity, Option<&PuzzleElement>), (Changed<Interaction>, With<ClickableNode>)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut player_sequence: ResMut<PlayerActivationSequence>,
    level_manager: Res<LevelManager>,
    // mut echo_path: ResMut<EchoPath>, // Echo path determination is complex and likely better handled in puzzle.rs or echo.rs
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let current_level_spec = level_manager.get_current_level();

    for (interaction, mut node_state, node_component, entity, _puzzle_element_opt) in &mut interaction_query {
        if *interaction == Interaction::Pressed { // Changed from Interaction::Clicked to Interaction::Pressed for more responsive feel
            match *node_state {
                NodeState::Idle | NodeState::Activating => {
                    *node_state = NodeState::Active;
                    player_sequence.add_activated_node(node_component.id);
                    info!("Node {:?} with ID {} activated.", entity, node_component.id);

                    // Start node activation logic is now primarily handled by echo::spawn_echo_system
                    // based on the NodeState::Active of the start node.
                }
                NodeState::Active => {
                    info!("Node {:?} is already active.", entity);
                }
                NodeState::Target | NodeState::Start => {
                    info!("Clicked on a Start/Target node: {:?}.", *node_state);
                }
            }
        }
    }
}

/// System to update the visual appearance of nodes based on their state.
pub fn update_node_visuals_system(
    mut query: Query<(&NodeState, &mut Sprite, Option<&PuzzleElement>), Changed<NodeState>>,
) {
    for (node_state, mut sprite, puzzle_opt) in &mut query {
        sprite.color = color_for_node_state(node_state);
        if puzzle_opt.is_some() {
            // Example: Make puzzle elements slightly larger or add an outline if possible with Sprite
            // sprite.custom_size = Some(Vec2::new(55.0, 55.0)); // Slightly larger
        } else {
            // sprite.custom_size = Some(Vec2::new(50.0, 50.0)); // Reset to default if not puzzle
        }
    }
}
File created successfully.
