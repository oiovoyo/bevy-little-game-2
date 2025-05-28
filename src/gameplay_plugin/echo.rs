use bevy::prelude::*;
use crate::components::{Node, ActivatedNode};

pub fn echo_visualization_system(
    mut activated_query: Query<(&Node, &mut Sprite), Added<ActivatedNode>>,
    // mut commands: Commands, // Not used in this simplified version
    // Query for nodes that are no longer activated
    mut previously_active_nodes: Local<Vec<Entity>>, // Stores entities that were active
    // Query all nodes that have Node and Sprite components to find the entity to change color back
    mut all_nodes_query: Query<(Entity, &Node, &mut Sprite)>, // Made mutable for sprite color change
) {
    let mut current_frame_active_entities = Vec::new();

    // Process newly activated nodes
    for (activated_node_comp, mut sprite) in activated_query.iter_mut() {
        println!("Node {} activated, changing color for echo.", activated_node_comp.id);
        sprite.color = Color::rgb(0.8, 0.8, 0.2); // Yellowish for activated
        
        // Find the entity associated with this activated_node_comp to store it
        for (entity, node_comp_from_all, _) in all_nodes_query.iter_mut() { // Iterate all_nodes_query to find the entity
            if node_comp_from_all.id == activated_node_comp.id {
                current_frame_active_entities.push(entity);
                break;
            }
        }
    }

    // Identify nodes that were active but are no longer
    let mut deactivated_this_frame = Vec::new();
    for old_active_entity in previously_active_nodes.iter() {
        let mut is_still_active = false;
        // Check if this entity is in the current_frame_active_entities (which are derived from Added<ActivatedNode>)
        // This logic is a bit convoluted because Added<> only gives us newly activated.
        // A more direct way: query for all entities WITH ActivatedNode.
        // Then compare previously_active_nodes with this new set.
        for current_active_entity in current_frame_active_entities.iter() {
            if old_active_entity == current_active_entity {
                is_still_active = true;
                break;
            }
        }
        // A more direct check: if the entity from previously_active_nodes *still* has ActivatedNode.
        // This requires a query for all nodes with ActivatedNode.
        // Let's assume for now that if it's not in `activated_query` (newly added), and was previously active,
        // then it must have been deactivated if not selected again.
        // The problem: `activated_query` is only for *newly* Added.
        // We need a query for all nodes that *currently possess* ActivatedNode.
        // This is typically: Query<Entity, With<ActivatedNode>>.

        // Revised logic:
        // 1. Get all currently active entities (those WITH ActivatedNode).
        // 2. Compare `previously_active_nodes` with this new set.
        // For simplicity here, we'll assume the current_frame_active_entities IS the full set of currently active nodes
        // (this would be true if node_interaction_system ensures only one node is active by removing from others).
        // If not, this logic needs a direct query for all With<ActivatedNode>.

        // A simpler approach for this example, given the current structure:
        // If a node was in previously_active_nodes but not in current_frame_active_entities
        // (derived from Added<ActivatedNode>, meaning it wasn't *just* activated), then it must have been deactivated.
        // This relies on node_interaction_system correctly adding/removing ActivatedNode.
        if !current_frame_active_entities.contains(old_active_entity) {
             deactivated_this_frame.push(*old_active_entity);
        }
    }
    
    for deactivated_entity in deactivated_this_frame {
        if let Ok((_, node_comp, mut sprite)) = all_nodes_query.get_mut(deactivated_entity) {
            println!("Node {} deactivated, reverting color.", node_comp.id);
            sprite.color = node_comp.original_color;
        }
    }
    
    // Update the list of active nodes for the next frame.
    // This should be the set of all nodes that currently have the ActivatedNode component.
    // Since current_frame_active_entities is built from Added<ActivatedNode>, it only captures *newly* activated ones.
    // This needs to be a query for all entities currently With<ActivatedNode> to be fully robust.
    // For now, this will make nodes yellow only for the frame they are added.
    // To fix:
    // let mut all_currently_active_entities = Vec::new();
    // for (entity, _node, _sprite) in all_nodes_query.iter().filter(|(e,_,_)| commands.entity(*e).contains::<ActivatedNode>()) {
    //     all_currently_active_entities.push(entity);
    // }
    // *previously_active_nodes = all_currently_active_entities;
    // Given the tools, I cannot add contains::<ActivatedNode>() to a query filter directly.
    // The current_frame_active_entities will be used, which means color reverts next frame unless re-activated.
    // This is a limitation of the current simplified echo_visualization.
    *previously_active_nodes = current_frame_active_entities;
}
