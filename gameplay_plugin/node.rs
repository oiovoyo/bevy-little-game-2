//! # Node Gameplay Logic

use bevy::prelude::*;
use crate::components::{Node, NodeState, ClickableNode, PuzzleElement};
use crate::resources::{PlayerActivationSequence, EchoPath, LevelManager};
use crate::game_state::GameState;
use super::color_for_node_state; // Accessing helper from parent module

/// System to handle player interaction with nodes (e.g., clicking).
pub fn node_interaction_system(
    mut interaction_query: Query<(&Interaction, &mut NodeState, &Node, Entity, Option<&PuzzleElement>), (Changed<Interaction>, With<ClickableNode>)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut player_sequence: ResMut<PlayerActivationSequence>,
    level_manager: Res<LevelManager>,
    mut echo_path: ResMut<EchoPath>, // To potentially trigger path recalculation or echo spawn
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let current_level_spec = level_manager.get_current_level();

    for (interaction, mut node_state, node_component, entity, puzzle_element_opt) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match *node_state {
                NodeState::Idle | NodeState::Activating => {
                    // Logic for activating a node
                    // For now, directly set to Active. In a real game, this might involve a timer or animation.
                    *node_state = NodeState::Active;
                    player_sequence.add_activated_node(node_component.id);
                    info!("Node {:?} with ID {} activated.", entity, node_component.id);

                    // If this is the start node and it's now active, prepare the echo path
                    if let Some(level) = current_level_spec {
                        if level.start_node_index == node_component.id && *node_state == NodeState::Active {
                             // Basic path: just start to target. Real game needs pathfinding based on active connections.
                            // This is a placeholder. Pathfinding should be more robust.
                            if let Some(target_node_entity) = find_node_entity_by_id(level.target_node_index, &interaction_query) {
                                // Simplified: direct path. Real game: A* or similar on active connections.
                                // For now, the echo will try to spawn towards the target if start is active.
                                // The actual path nodes are determined in echo::spawn_echo_system
                                info!("Start node {} activated. Echo path will be determined.", node_component.id);
                            }
                        }
                    }

                }
                NodeState::Active => {
                    // Node is already active, maybe some other interaction or none.
                    info!("Node {:?} is already active.", entity);
                }
                NodeState::Target | NodeState::Start => {
                    // Target and Start nodes might have different interaction rules, or none after initial setup.
                    info!("Clicked on a Start/Target node: {:?}.", *node_state);
                }
            }
        }
    }
}

// Helper function to find a node entity by its ID from query results
// This is inefficient for large numbers of nodes and interactions; consider a resource map.
fn find_node_entity_by_id<'a, 'b>(
    id_to_find: usize,
    query: &'a Query<(&Interaction, &mut NodeState, &Node, Entity, Option<&PuzzleElement>), (Changed<Interaction>, With<ClickableNode>)>
) -> Option<Entity> {
    for (_, _, node, entity, _) in query.iter() {
        if node.id == id_to_find {
            return Some(entity);
        }
    }
    None
}


/// System to update the visual appearance of nodes based on their state.
pub fn update_node_visuals_system(
    mut query: Query<(&NodeState, &mut Sprite, Option<&PuzzleElement>), Changed<NodeState>>,
    // Potentially add assets here if using different sprites for states
) {
    for (node_state, mut sprite, puzzle_opt) in &mut query {
        sprite.color = color_for_node_state(node_state);
        // Additional visual changes based on state or if it's a puzzle element
        if puzzle_opt.is_some() {
            // Maybe add an outline or different shape/size for puzzle elements
        }
    }
}
File created successfully.
