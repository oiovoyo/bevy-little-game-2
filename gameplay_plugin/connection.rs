//! # Connection Gameplay Logic

use bevy::prelude::*;
use crate::components::{Connection, Node, NodeState};

/// System to draw connections between nodes.
/// This is a placeholder. In a real game, this might use bevy_prototype_lyon for drawing lines,
/// or custom meshes, or even Bevy's built-in Gizmos for a simpler look.
pub fn draw_connections_system(
    mut gizmos: Gizmos,
    connection_query: Query<&Connection>,
    node_query: Query<(&Transform, &NodeState, &Node), With<Node>>, // Query Node to get positions and states
) {
    for connection in connection_query.iter() {
        // Get positions of the two connected nodes
        let pos_a = node_query.get(connection.node_a).map(|(t, _, _)| t.translation.truncate());
        let pos_b = node_query.get(connection.node_b).map(|(t, _, _)| t.translation.truncate());
        
        // Get states of the two connected nodes
        let state_a_active = node_query.get(connection.node_a)
            .map_or(false, |(_, state, _)| *state == NodeState::Active || *state == NodeState::Start || *state == NodeState::Target);
        let state_b_active = node_query.get(connection.node_b)
            .map_or(false, |(_, state, _)| *state == NodeState::Active || *state == NodeState::Start || *state == NodeState::Target);

        if let (Ok(pos1), Ok(pos2)) = (pos_a, pos_b) {
            let color = if connection.is_active || (state_a_active && state_b_active) {
                Color::YELLOW // Active connection or both nodes active
            } else if state_a_active || state_b_active {
                Color::ORANGE // One node active, connection potentially usable
            }
            else {
                Color::GRAY // Dormant connection
            };
            gizmos.line_2d(pos1, pos2, color);
        }
    }
}

// Future system: Update Connection `is_active` field based on echo path or player actions.
// This would then influence the color in `draw_connections_system`.
// pub fn update_connection_active_state_system(
//     mut connection_query: Query<&mut Connection>,
//     echo_path: Res<EchoPath>, // Or other logic
// ) {
//     // ... logic to determine which connections are part of an active path ...
// }
File created successfully.
