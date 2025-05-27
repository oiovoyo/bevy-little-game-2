//! # Connection Gameplay Logic

use bevy::prelude::*;
use crate::components::{Connection, Node, NodeState};

/// System to draw connections between nodes.
/// This uses Bevy's built-in Gizmos for a simpler look.
pub fn draw_connections_system(
    mut gizmos: Gizmos,
    connection_query: Query<&Connection>,
    node_query: Query<(&Transform, &NodeState, &Node), With<Node>>, // Query Node to get positions and states
) {
    for connection in connection_query.iter() {
        let pos_a_result = node_query.get(connection.node_a);
        let pos_b_result = node_query.get(connection.node_b);

        if let (Ok((transform_a, state_a, _)), Ok((transform_b, state_b, _))) = (pos_a_result, pos_b_result) {
            let pos1 = transform_a.translation.truncate();
            let pos2 = transform_b.translation.truncate();
            
            let state_a_is_conductive = *state_a == NodeState::Active || *state_a == NodeState::Start || *state_a == NodeState::Target;
            let state_b_is_conductive = *state_b == NodeState::Active || *state_b == NodeState::Start || *state_b == NodeState::Target;

            let color = if connection.is_active || (state_a_is_conductive && state_b_is_conductive) {
                Color::YELLOW // Active connection or both nodes are in a state that can conduct
            } else if state_a_is_conductive || state_b_is_conductive {
                Color::ORANGE // One node is conductive, connection potentially usable
            }
            else {
                Color::GRAY // Dormant connection
            };
            gizmos.line_2d(pos1, pos2, color);
        }
    }
}

// Future system: Update Connection `is_active` field based on echo path or player actions.
// pub fn update_connection_active_state_system(
//     mut connection_query: Query<(&mut Connection, Entity)>,
//     echo_path: Res<EchoPath>, // Assuming EchoPath stores pairs of connected node Entities
// ) {
//     // First, reset all connections to inactive if that's the desired logic
//     // for c in connection_query.iter_mut() { c.0.is_active = false; }

//     // Then, iterate through the path and mark connections as active
//     // This depends on how EchoPath is structured. If it's a list of node entities:
//     // for N in 0..echo_path.path_node_entities.len() -1 {
//     //    let node1_entity = echo_path.path_node_entities[N];
//     //    let node2_entity = echo_path.path_node_entities[N+1];
//     //    for (mut conn, conn_entity) in connection_query.iter_mut() {
//     //        if (conn.node_a == node1_entity && conn.node_b == node2_entity) ||
//     //           (conn.node_a == node2_entity && conn.node_b == node1_entity) {
//     //            conn.is_active = true;
//     //            break;
//     //        }
//     //    }
//     // }
// }
File created successfully.
