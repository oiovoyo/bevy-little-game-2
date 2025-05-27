//! # Puzzle Logic and Validation

use bevy::prelude::*;
use crate::components::{Node, NodeState, DataEcho, PuzzleElement};
use crate::resources::{LevelManager, GameTimer, PlayerActivationSequence, EchoPath};
use crate::game_state::GameState;

/// System to check if the level completion conditions are met.
pub fn check_level_completion_system(
    mut game_state: ResMut<NextState<GameState>>,
    level_manager: Res<LevelManager>,
    player_sequence: Res<PlayerActivationSequence>,
    node_query: Query<(Entity, &Node, &NodeState)>, // Querying Entity as well for robust checks
    echo_query: Query<&DataEcho>,
) {
    let current_level = if let Some(l) = level_manager.get_current_level() {
        l
    } else {
        return; 
    };

    // Find the Entity of the target node for the current level
    let target_node_entity_opt = node_query.iter()
        .find(|(_, node_c, _)| node_c.id == current_level.target_node_index)
        .map(|(entity, _, _)| entity);

    if target_node_entity_opt.is_none() {
        warn!("Target node ID {} not found as an entity in the current level.", current_level.target_node_index);
        return;
    }
    let target_node_entity = target_node_entity_opt.unwrap();

    // Condition 1: Has an echo reached THE target node for the level?
    let mut echo_reached_final_target = false;
    for echo in echo_query.iter() {
        if echo.current_node == target_node_entity && // current_node is an Entity
           echo.target_node == target_node_entity && // echo's specific target is the level's target
           echo.current_segment_index + 1 >= echo.path.len() { // and it has finished its path
            echo_reached_final_target = true;
            break;
        }
    }
    
    // Condition 2: Is the target node itself in an Active state?
    // (This might be redundant if echo arrival implies activation, but can be an explicit check)
    let mut target_node_component_active = false;
    if let Ok((_, _, state)) = node_query.get(target_node_entity) {
        if *state == NodeState::Active || *state == NodeState::Target { // Target state might also count as "active" for completion
            target_node_component_active = true;
        }
    }

    // Condition 3: Are all puzzle requirements met (e.g., activation sequence)?
    let mut sequence_correct_and_complete = true; // Assume true if no sequence is required
    if let Some(required_sequence) = &current_level.required_activation_sequence {
        if player_sequence.activated_node_ids.len() < required_sequence.len() {
            sequence_correct_and_complete = false; // Not all required nodes activated yet
        } else {
            // Check if the player's activation order matches the required sequence exactly
            // (or if it's a prefix and length matches)
            for (i, &req_id) in required_sequence.iter().enumerate() {
                if player_sequence.activated_node_ids.get(i) != Some(&req_id) {
                    sequence_correct_and_complete = false;
                    break;
                }
            }
            // Ensure no extra nodes were activated if the sequence must be exact and all are activated
            if sequence_correct_and_complete && player_sequence.activated_node_ids.len() > required_sequence.len() {
               // This depends on game rules. For a strict sequence, this would be a fail.
               // For now, let's assume if the required sequence is a prefix and met, it's okay.
               // However, for "level completion", usually exact match of requirements is needed.
               // Let's assume for now that if a sequence is specified, its length must also match.
               if player_sequence.activated_node_ids.len() != required_sequence.len() {
                    sequence_correct_and_complete = false;
               }
            }
        }
    }
    
    // Check PuzzleElement conditions (e.g. all nodes with PuzzleElement having required_order are active in that order)
    // This part can be complex. For now, the `required_activation_sequence` in `Level` is the main driver.
    // A more detailed check would iterate PuzzleElements and verify their specific conditions against PlayerActivationSequence.

    if echo_reached_final_target && target_node_component_active && sequence_correct_and_complete {
        info!("Level {} complete!", current_level.id);
        game_state.set(GameState::LevelComplete);
    }
}

/// System to check for level failure conditions (e.g., timer runs out).
pub fn check_level_fail_system(
    mut game_state: ResMut<NextState<GameState>>,
    game_timer: Res<GameTimer>,
    level_manager: Res<LevelManager>,
    player_sequence: Res<PlayerActivationSequence>, // To check for incorrect activations
) {
    // Check for timer expiration
    if game_timer.time_limit_seconds.is_some() && game_timer.timer.finished() && !game_timer.timer.just_finished() {
        // Check !just_finished to prevent race condition if completion and timer end on same frame
        if game_state.0 != Some(GameState::LevelComplete) && game_state.0 != Some(GameState::GameWon) { // Don't fail if already completed
            info!("Level failed: Timer ran out.");
            game_state.set(GameState::GameOver);
            return;
        }
    }

    // Check for incorrect sequence activation if the level defines a strict sequence
    // This needs to be carefully designed: when is an activation "wrong"?
    // - Activating a node not in the sequence at all?
    // - Activating a node in the sequence but out of order?
    if let Some(current_level) = level_manager.get_current_level() {
        if let Some(required_sequence) = &current_level.required_activation_sequence {
            // Iterate through player's activations so far
            for (i, activated_node_id) in player_sequence.activated_node_ids.iter().enumerate() {
                // If the current activated node is beyond the length of required sequence, it's an extra activation.
                // (This might be fine or a failure depending on rules - for now, let's assume it's fine if prefix matches)
                if i >= required_sequence.len() { 
                    // Potentially a failure condition if sequence must be exact length
                    // Example: if required is [0,1,2] and player does [0,1,2,3] -> this is an extra activation.
                    // For now, we only check if the *prefix* is wrong.
                    continue;
                }

                if required_sequence[i] != *activated_node_id {
                    // Player activated a node that doesn't match the required node at this position in the sequence.
                    info!(
                        "Level failed: Incorrect node activation. Expected node ID {} at position {}, but got {}.",
                        required_sequence[i], i, activated_node_id
                    );
                    // game_state.set(GameState::GameOver); // This might be too strict depending on puzzle design.
                                                          // Some puzzles might allow trying different paths.
                                                          // Failure should be based on definitive game rules (e.g. echo hits wrong target, or specific "trap" node).
                    return;
                }
            }
        }
    }
}
File created successfully.
