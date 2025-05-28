use bevy::prelude::*;
use crate::components::{Node, ActivatedNode};
// use crate::game_state::GameState; // Not directly used here currently

pub fn node_interaction_system(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    node_query: Query<(Entity, &Transform, &Node), Without<ActivatedNode>>, // Only non-activated
    // activated_node_query: Query<Entity, With<ActivatedNode>>, // Used for deselection logic in connection
    mut selected_node_entity: Local<Option<Entity>>, 
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = camera_q.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Check if we are clicking an existing node to activate it
            let mut clicked_on_node = false;
            for (node_entity, node_transform, node_comp) in node_query.iter() {
                let distance = world_position.distance(node_transform.translation.truncate());
                if distance < 25.0 { // Node radius
                    println!("Clicked node to activate: {}", node_comp.id);
                    
                    // Deselect any previously selected node if it's different
                    if let Some(prev_selected) = *selected_node_entity {
                        if prev_selected != node_entity {
                             commands.entity(prev_selected).remove::<ActivatedNode>();
                        }
                    }
                    
                    commands.entity(node_entity).insert(ActivatedNode);
                    *selected_node_entity = Some(node_entity);
                    clicked_on_node = true;
                    break; 
                }
            }
            // If we clicked but not on a node, and a node was selected, deselect it (unless dragging starts)
            if !clicked_on_node {
                if let Some(prev_selected) = *selected_node_entity {
                     // This deselection will be handled by connection drawing logic if a drag starts
                     // otherwise, it might be desired to deselect on empty space click.
                     // For now, connection logic handles deselection.
                }
                 *selected_node_entity = None; // Clear selection if clicked on empty space
            }
        }
    }
}
