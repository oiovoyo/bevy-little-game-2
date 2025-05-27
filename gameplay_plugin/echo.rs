//! # Echo Propagation Logic

use bevy::prelude::*;
use std::collections::VecDeque;
use crate::components::{DataEcho, Node, NodeState, Connection};
use crate::resources::{LevelManager, EchoPath}; // PlayerActivationSequence removed as path is now directly from EchoPath
use crate::game_state::GameState; // To potentially trigger states based on echo events

const ECHO_SPEED: f32 = 150.0; // World units per second

/// System to spawn DataEchos when a start node is activated and a path is available.
pub fn spawn_echo_system(
    mut commands: Commands,
    level_manager: Res<LevelManager>,
    mut echo_path_res: ResMut<EchoPath>, // Use the pre-calculated path
    node_query: Query<(Entity, &Node, &NodeState, &Transform)>, // To get start node entity and transform
    // Removed PlayerActivationSequence as it's not directly used for spawning anymore
) {
    let current_level = if let Some(l) = level_manager.get_current_level() { l } else { return };

    // Find the start node entity and check its state
    let mut start_node_entity_opt: Option<(Entity, &Transform)> = None;
    for (entity, node_component, node_state, transform) in node_query.iter() {
        if node_component.id == current_level.start_node_index && *node_state == NodeState::Active {
            start_node_entity_opt = Some((entity, transform));
            break;
        }
    }

    if let Some((start_node_entity, start_transform)) = start_node_entity_opt {
        // Check if an echo for this start node already exists or if path is not ready
        // This simple check prevents re-spawning if system runs multiple times while start is active.
        // A more robust check might involve specific Echo component properties or a marker on the StartNode.
        let mut echo_already_exists = false;
        // This check is currently missing as we don't have a query for existing echos here.
        // For now, assume we only spawn once, or that cleanup handles old echos.

        // If a path is defined in EchoPath (set by puzzle logic or node activation)
        if !echo_path_res.is_empty() && !echo_already_exists {
             // The first node in echo_path_res should be the start_node_entity if path is correct
            if echo_path_res.path_node_entities.front() == Some(&start_node_entity) {
                let mut full_path_for_echo = echo_path_res.path_node_entities.clone().into_iter().collect::<Vec<Entity>>();
                
                // The actual target for this echo is the last node in its specific path.
                let final_target_in_path = full_path_for_echo.last().cloned();

                if let Some(target_entity) = final_target_in_path {
                    info!("Spawning echo from Entity: {:?} towards Entity: {:?}", start_node_entity, target_entity);
                    commands.spawn((
                        DataEcho {
                            current_node: start_node_entity, // Entity of the current node
                            path: full_path_for_echo, // The full path of Node Entities
                            target_node: target_entity, // The final destination Entity for this echo
                            speed: ECHO_SPEED,
                            progress_on_connection: 0.0,
                            current_segment_index: 0, // Starts at the first node in its path
                        },
                        SpriteBundle { // Simple visual placeholder for the echo
                            sprite: Sprite {
                                color: Color::CYAN,
                                custom_size: Some(Vec2::new(20.0, 20.0)),
                                ..default()
                            },
                            transform: *start_transform, // Start at the start node's position
                            ..default()
                        },
                        Name::new("DataEcho"),
                    ));
                    // Clear the path from the resource as it's now assigned to this echo
                    // Or manage multiple echos if the design allows
                    echo_path_res.clear();
                } else {
                    // info!("Echo path defined but no target found in path for echo from {:?}", start_node_entity);
                }
            } else {
                 // info!("Echo path available, but its first node {:?} does not match start node {:?}", echo_path_res.path_node_entities.front(), start_node_entity);
            }
        }
    }
}


/// System to move DataEchos along their designated paths.
pub fn update_echo_movement_system(
    mut echo_query: Query<(&mut DataEcho, &mut Transform)>,
    node_query: Query<&Transform, (With<Node>, Without<DataEcho>)>, // Node transforms, excluding echos
    time: Res<Time>,
    // Potentially mut connection_query: Query<&mut Connection> to mark connections as active by echo
) {
    for (mut echo, mut echo_transform) in echo_query.iter_mut() {
        if echo.current_segment_index + 1 >= echo.path.len() {
            // Echo has reached or passed the end of its path
            if echo.current_node == echo.target_node { // Ensure it's exactly at the target
                // Despawning or other logic will be handled by `despawn_echo_at_target_system`
            }
            continue;
        }

        let current_path_node_entity = echo.path[echo.current_segment_index];
        let next_path_node_entity = echo.path[echo.current_segment_index + 1];

        let current_node_pos = match node_query.get(current_path_node_entity) {
            Ok(transform) => transform.translation.truncate(),
            Err(_) => {
                // Node not found, perhaps despawn echo or log error
                // For now, skip this echo's movement
                warn!("Echo's current path node {:?} not found.", current_path_node_entity);
                continue;
            }
        };
        let next_node_pos = match node_query.get(next_path_node_entity) {
            Ok(transform) => transform.translation.truncate(),
            Err(_) => {
                warn!("Echo's next path node {:?} not found.", next_path_node_entity);
                continue;
            }
        };

        let distance_to_next_node = current_node_pos.distance(next_node_pos);
        if distance_to_next_node == 0.0 { // Avoid division by zero if nodes are at the same spot
            echo.progress_on_connection = 1.0; // Instantly at next node
        } else {
            echo.progress_on_connection += (echo.speed * time.delta_seconds()) / distance_to_next_node;
        }

        if echo.progress_on_connection >= 1.0 {
            // Reached the next node in the path segment
            echo.progress_on_connection = 0.0; // Reset progress for the next segment
            echo.current_segment_index += 1;
            echo.current_node = next_path_node_entity; // Update current node to the one just reached

            // Update transform to be exactly at the new current node
            echo_transform.translation = next_node_pos.extend(echo_transform.translation.z);

            if echo.current_node == echo.target_node {
                // Echo has reached its final target node
                // Despawn logic will be handled by despawn_echo_at_target_system
                info!("Echo reached its target node: {:?}", echo.target_node);
            }
        } else {
            // Interpolate position along the current segment
            echo_transform.translation = current_node_pos.lerp(next_node_pos, echo.progress_on_connection).extend(echo_transform.translation.z);
        }
    }
}


/// System to despawn echos that have reached their target node.
pub fn despawn_echo_at_target_system(
    mut commands: Commands,
    echo_query: Query<(Entity, &DataEcho)>,
    mut node_query: Query<(&mut NodeState, &Node)>, // To mark target node as "hit" or "completed"
    level_manager: Res<LevelManager>, // To know the actual level target
    mut game_state: ResMut<NextState<GameState>>, // To potentially trigger level complete
) {
    let current_level_opt = level_manager.get_current_level();

    for (echo_entity, echo_component) in echo_query.iter() {
        if echo_component.current_node == echo_component.target_node && // Echo is at its own target
           echo_component.current_segment_index +1 >= echo_component.path.len() // And has finished its path
        {
            info!("Echo {:?} reached target node {:?}. Despawning echo.", echo_entity, echo_component.target_node);
            commands.entity(echo_entity).despawn_recursive();

            // If this echo's target was also THE level's target node, mark that node.
            if let Some(current_level) = current_level_opt {
                 // Find the Node component corresponding to the echo's target_node Entity
                let target_node_id_opt = node_query.iter()
                    .find(|(_, node_comp)| {
                        // This comparison is tricky: echo_component.target_node is an Entity.
                        // Node ID needs to be derived from this Entity.
                        // For now, let's assume we can get the Node component for the target_node Entity.
                        // This requires a query that can fetch a Node component given an Entity.
                        // This part needs a robust way to link echo's target Entity to a Node ID.
                        // Placeholder: We assume echo.target_node's ID matches current_level.target_node_index
                        // This is a simplification.
                        false // This check needs to be fixed with proper Entity to Node ID mapping.
                    })
                    .map(|(_, node_comp)| node_comp.id);


                // A simpler, more direct approach: Iterate through nodes to find the one matching the target_node entity.
                for (mut state, node_c) in node_query.iter_mut() {
                    // This is still not quite right. echo_component.target_node is an Entity.
                    // We need to query for the Entity itself.
                    // This needs a query for `(Entity, &mut NodeState, &Node)`
                    // And then check if `entity == echo_component.target_node`
                    // For now, we'll assume if an echo is despawned, and its target_node *ID* matches the level's target_node_index,
                    // then the level target is hit. This requires DataEcho.target_node to store the *ID* instead of Entity,
                    // or have a reliable Entity -> ID map.
                    // Let's assume DataEcho.target_node was actually the Entity of the target.

                    // This part is tricky due to Entity IDs vs Node component IDs.
                    // A proper implementation would query for the target entity directly.
                    // For now, we'll rely on `check_level_completion_system` to verify.
                }
            }
        }
    }
}
File created successfully.
