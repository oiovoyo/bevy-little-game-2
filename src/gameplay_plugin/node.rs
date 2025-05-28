use bevy::prelude::*;
use crate::components::{Node, ActivatedNode};

pub fn node_interaction_system(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>, // Changed to Query<&Window>
    camera_q: Query<(&Camera, &GlobalTransform)>,
    node_query: Query<(Entity, &Transform, &Node), Without<ActivatedNode>>,
    mut selected_node_entity: Local<Option<Entity>>, 
) {
    // Get the primary window. If it doesn't exist, exit.
    let Ok(window) = windows.single() else { 
        // Consider logging an error or handling more gracefully if multiple windows or no window is an expected state.
        return; 
    };
    
    // Get the camera. If it doesn't exist, exit.
    let Ok((camera, camera_transform)) = camera_q.single() else { 
        // Consider logging an error.
        return; 
    };

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(world_position) = window.cursor_position() // `window` is now &Window
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            let mut clicked_on_node = false;
            for (node_entity, node_transform, node_comp) in node_query.iter() {
                let distance = world_position.distance(node_transform.translation.truncate());
                if distance < 25.0 { // Node radius
                    println!("Clicked node to activate: {}", node_comp.id);
                    
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
            if !clicked_on_node {
                if let Some(prev_selected) = *selected_node_entity {
                    // If clicked empty space, and a node was selected, deselect it.
                    commands.entity(prev_selected).remove::<ActivatedNode>();
                }
                *selected_node_entity = None; 
            }
        }
    }
}
