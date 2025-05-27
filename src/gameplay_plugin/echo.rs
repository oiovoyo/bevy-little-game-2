//! # Echo Propagation Logic

use bevy::prelude::*;
use std::collections::VecDeque;
use crate::components::{DataEcho, Node, NodeState, Connection}; // Assuming Connection might be used later for path validation
use crate::resources::{LevelManager, EchoPath};
// use crate::game_state::GameState; // Not directly used here

const ECHO_SPEED: f32 = 150.0; // World units per second

/// System to spawn DataEchos when a start node is activated and a path is available.
pub fn spawn_echo_system(
    mut commands: Commands,
    level_manager: Res<LevelManager>,
    mut echo_path_res: ResMut<EchoPath>, 
    node_query: Query<(Entity, &Node, &NodeState, &Transform)>,
    existing_echo_query: Query<(), With<DataEcho>>, // Query to check if an echo already exists
) {
    let current_level = if let Some(l) = level_manager.get_current_level() { l } else { return };

    // Check if an echo already exists. If so, don't spawn another.
    // This is a simple way to prevent multiple echos for the same start event.
    // More complex logic might allow multiple echos or specific conditions for new echos.
    if !existing_echo_query.is_empty() {
        return;
    }

    let mut start_node_entity_opt: Option<(Entity, &Transform)> = None;
    let mut target_node_entity_opt: Option<Entity> = None;
    let mut path_node_entities: Vec<Entity> = Vec::new();


    // Find the start node entity and check its state
    for (entity, node_component, node_state, transform) in node_query.iter() {
        if node_component.id == current_level.start_node_index {
            if *node_state == NodeState::Active || *node_state == NodeState::Start { // Start node must be active/start
                 start_node_entity_opt = Some((entity, transform));
            }
        }
        if node_component.id == current_level.target_node_index {
            target_node_entity_opt = Some(entity);
        }
    }
    
    // Reconstruct path based on player's activation sequence if no pre-defined path is set by puzzle logic
    // This is a simplified pathfinding: assumes nodes in PlayerActivationSequence form the path.
    // A real game would use A* or similar on the connection graph.
    if echo_path_res.is_empty() {
        if let Some((start_entity, _)) = start_node_entity_opt {
            let mut potential_path = Vec::new();
            let mut current_path_node_id = current_level.start_node_index;
            let mut path_found = false;

            // This is a placeholder for actual pathfinding.
            // It assumes a linear activation or a pre-defined correct sequence from resources.
            // For a robust solution, after player activates nodes, a pathfinding algorithm
            // should run on the *active* connection graph.
            // Here, we'll just make a direct path from start to target if both are active,
            // and assume intermediate nodes will be activated by the player.
            // This part needs to be much more intelligent in a real game.

            // Simplified: if start node is active, and we have a target, try to make a path.
            // The actual path used by the echo will be from `echo_path_res` if populated by puzzle logic,
            // otherwise this basic spawn won't happen unless `echo_path_res` is explicitly set.
            // For the purpose of this example, let's assume puzzle logic (e.g. node activation)
            // should populate `echo_path_res`. This system just consumes it.
        }
    }


    if let Some((start_node_entity, start_transform)) = start_node_entity_opt {
        if !echo_path_res.is_empty() {
            // Ensure the path starts with the actual start_node_entity
            if echo_path_res.path_node_entities.front() == Some(&start_node_entity) {
                let full_path_for_echo: Vec<Entity> = echo_path_res.path_node_entities.iter().cloned().collect();
                
                let final_target_in_path = full_path_for_echo.last().cloned();

                if let Some(target_entity) = final_target_in_path {
                    info!("Spawning echo from Entity: {:?} towards Entity: {:?} via path: {:?}", start_node_entity, target_entity, full_path_for_echo);
                    commands.spawn((
                        DataEcho {
                            current_node: start_node_entity,
                            path: full_path_for_echo,
                            target_node: target_entity, 
                            speed: ECHO_SPEED,
                            progress_on_connection: 0.0,
                            current_segment_index: 0,
                        },
                        SpriteBundle { 
                            sprite: Sprite {
                                color: Color::CYAN,
                                custom_size: Some(Vec2::new(20.0, 20.0)),
                                ..default()
                            },
                            transform: *start_transform, 
                            ..default()
                        },
                        Name::new("DataEcho"),
                    ));
                    echo_path_res.clear(); // Consume the path
                }
            } else {
                // Path doesn't start at the designated start node, clear it to prevent incorrect spawning.
                if !echo_path_res.is_empty() { // Check again because it might have been cleared by another system
                    // info!("Path in EchoPath resource does not start with the current start node. Path: {:?}, Start Node: {:?}", echo_path_res.path_node_entities, start_node_entity);
                    // echo_path_res.clear(); // Optional: clear if path is invalid for current start node.
                }
            }
        }
    }
}


/// System to move DataEchos along their designated paths.
pub fn update_echo_movement_system(
    mut echo_query: Query<(&mut DataEcho, &mut Transform)>,
    node_transforms: Query<&Transform, (With<Node>, Without<DataEcho>)>, // Node transforms, excluding echos
    time: Res<Time>,
    mut connection_query: Query<&mut Connection>, // To mark connections as active
) {
    for (mut echo, mut echo_transform) in echo_query.iter_mut() {
        if echo.current_segment_index + 1 >= echo.path.len() {
            // Echo has completed its path. Despawn will be handled by another system.
            continue;
        }

        let current_path_node_entity = echo.path[echo.current_segment_index];
        let next_path_node_entity = echo.path[echo.current_segment_index + 1];

        // Mark the connection as active (if not already)
        for mut conn in connection_query.iter_mut() {
            if (conn.node_a == current_path_node_entity && conn.node_b == next_path_node_entity) ||
               (conn.node_a == next_path_node_entity && conn.node_b == current_path_node_entity) {
                if !conn.is_active {
                    conn.is_active = true;
                }
                break;
            }
        }


        let current_node_pos_opt = node_transforms.get(current_path_node_entity).ok();
        let next_node_pos_opt = node_transforms.get(next_path_node_entity).ok();

        if let (Some(current_node_transform), Some(next_node_transform)) = (current_node_pos_opt, next_node_pos_opt) {
            let current_node_pos = current_node_transform.translation.truncate();
            let next_node_pos = next_node_transform.translation.truncate();

            let distance_to_next_node = current_node_pos.distance(next_node_pos);
            if distance_to_next_node < f32::EPSILON { // If nodes are practically at the same spot
                echo.progress_on_connection = 1.0; // Consider segment traversed
            } else {
                echo.progress_on_connection += (echo.speed * time.delta_seconds()) / distance_to_next_node;
            }

            if echo.progress_on_connection >= 1.0 {
                echo.progress_on_connection = 0.0; // Reset for next segment
                echo.current_segment_index += 1;
                echo.current_node = next_path_node_entity; 

                echo_transform.translation = next_node_pos.extend(echo_transform.translation.z);

                if echo.current_node == echo.target_node {
                    info!("Echo reached its target node: {:?}", echo.target_node);
                    // Despawn logic handled by `despawn_echo_at_target_system`
                }
            } else {
                echo_transform.translation = current_node_pos.lerp(next_node_pos, echo.progress_on_connection).extend(echo_transform.translation.z);
            }
        } else {
            // Node not found, path is broken.
            // Consider despawning echo or logging an error.
            warn!("Echo path node not found. Current: {:?}, Next: {:?}", current_path_node_entity, next_path_node_entity);
            // Optionally, despawn the echo here if its path is invalid.
            // commands.entity(echo_entity_id).despawn(); // Need echo_entity_id here
        }
    }
}


/// System to despawn echos that have reached their target node.
pub fn despawn_echo_at_target_system(
    mut commands: Commands,
    echo_query: Query<(Entity, &DataEcho)>,
    // node_query might be needed if target node needs state change upon echo arrival
    // mut node_query: Query<(&mut NodeState, &Node)>, 
    // level_manager: Res<LevelManager>, 
    // mut game_state: ResMut<NextState<GameState>>,
) {
    for (echo_entity, echo_component) in echo_query.iter() {
        // Check if the echo has reached the last node in its specific path
        if echo_component.current_segment_index + 1 >= echo_component.path.len() &&
           echo_component.current_node == echo_component.target_node {
            info!("Echo {:?} reached its target node {:?}. Despawning echo.", echo_entity, echo_component.target_node);
            commands.entity(echo_entity).despawn_recursive();

            // Note: Level completion logic is handled in `puzzle::check_level_completion_system`.
            // That system will verify if this echo reaching its target fulfills level conditions.
        }
    }
}
File created successfully.
