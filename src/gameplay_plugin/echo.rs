use bevy::prelude::*; // Added
use crate::components::{Node, ActivatedNode};

pub fn echo_visualization_system(
    mut activated_query: Query<(&Node, &mut Sprite), Added<ActivatedNode>>,
    mut previously_active_nodes: Local<Vec<Entity>>, 
    all_nodes_query: Query<(Entity, &Node, &mut Sprite)>, 
) {
    let mut current_frame_newly_activated_entities = Vec::new();

    for (activated_node_comp, mut sprite) in activated_query.iter_mut() {
        println!("Node {} activated, changing color for echo.", activated_node_comp.id);
        sprite.color = Color::srgb(0.8, 0.8, 0.2); // Corrected: Yellowish for activated
        
        for (entity, node_comp_from_all, _) in all_nodes_query.iter() {
            if node_comp_from_all.id == activated_node_comp.id {
                current_frame_newly_activated_entities.push(entity);
                break; 
            }
        }
    }

    let mut still_active_from_previous = Vec::new();
    for old_active_entity in previously_active_nodes.iter() {
        let mut still_has_activated_tag = false;
        // Check if this old_active_entity is still considered active by some means
        // (e.g. if it's in current_frame_newly_activated_entities, or if we had a query for With<ActivatedNode>)
        // For this example, we assume if it's not in current_frame_newly_activated_entities, it might have been deactivated
        if current_frame_newly_activated_entities.contains(old_active_entity) {
            still_has_activated_tag = true;
        } else {
            // If not newly activated, we infer it might have been deactivated if it was yellow.
            // This is imperfect. A RemovedComponents<ActivatedNode> system is better.
        }

        if still_has_activated_tag {
            still_active_from_previous.push(*old_active_entity);
        } else {
             // If it's not in current_frame_newly_activated_entities, it means ActivatedNode was NOT just added.
             // If it was yellow (implying it had ActivatedNode), then it must have been removed.
            if let Ok((_, node_comp, mut sprite)) = all_nodes_query.get(*old_active_entity) {
                // Only revert if it was actually yellow (activated color)
                if sprite.color == Color::srgb(0.8, 0.8, 0.2) {
                    println!("Node {} presumed deactivated (was yellow, not newly activated), reverting color.", node_comp.id);
                    sprite.color = node_comp.original_color;
                } else {
                    // It wasn't yellow, so it might have been deactivated already or never activated.
                    // Or it's active but wasn't caught by Added query this frame.
                    still_active_from_previous.push(*old_active_entity); // Keep it if it's not yellow
                }
            }
        }
    }
    
    // The new list of "active" nodes (for color purposes) are those that were just activated
    *previously_active_nodes = current_frame_newly_activated_entities;
}
