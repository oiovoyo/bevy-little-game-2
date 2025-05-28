use bevy::prelude::*;
use crate::components::{Node, Connection, ActivatedNode};
use crate::resources::PlayerAttempt;
use super::ConnectionAttemptEvent; 

#[derive(Resource, Default)]
struct DragState {
    start_node_entity: Option<Entity>,
    start_node_id: Option<usize>, // Store ID for event
    current_mouse_pos: Vec2,
}

pub fn draw_connection_system(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    node_query: Query<(Entity, &Transform, &Node)>, 
    activated_q: Query<(Entity, &Node), With<ActivatedNode>>, // To get the node that IS activated
    mut drag_state: Local<DragState>,
    mut gizmos: Gizmos, 
    mut connection_attempt_writer: EventWriter<ConnectionAttemptEvent>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        drag_state.current_mouse_pos = world_pos;

        if mouse_button_input.just_pressed(MouseButton::Left) {
            // Try to start a drag IF an activated node is clicked
            // This assumes node_interaction_system runs before and sets ActivatedNode
            if let Ok((activated_entity, activated_node_comp)) = activated_q.get_single() {
                 if let Ok((_, activated_node_transform, _)) = node_query.get(activated_entity) {
                    let distance = world_pos.distance(activated_node_transform.translation.truncate());
                    if distance < 25.0 { // Clicked on the already activated node
                        drag_state.start_node_entity = Some(activated_entity);
                        drag_state.start_node_id = Some(activated_node_comp.id);
                        // Snap start to node center for gizmo drawing
                        drag_state.current_mouse_pos = activated_node_transform.translation.truncate(); 
                        println!("Connection drag started from activated node: {}", activated_node_comp.id);
                    }
                 }
            }
        }

        if mouse_button_input.pressed(MouseButton::Left) {
            if let Some(start_entity_val) = drag_state.start_node_entity {
                if let Ok((_, start_node_transform, _)) = node_query.get(start_entity_val) {
                     gizmos.line_2d(start_node_transform.translation.truncate(), drag_state.current_mouse_pos, Color::YELLOW);
                } else { // Start node entity is no longer valid (e.g. despawned)
                    drag_state.start_node_entity = None;
                    drag_state.start_node_id = None;
                }
            }
        }

        if mouse_button_input.just_released(MouseButton::Left) {
            if let (Some(start_entity_val), Some(start_node_id_val)) = (drag_state.start_node_entity, drag_state.start_node_id) {
                let mut end_node_found = false;
                for (end_entity, end_node_transform, end_node_comp) in node_query.iter() {
                    if start_entity_val == end_entity { continue; } 

                    let distance = world_pos.distance(end_node_transform.translation.truncate());
                    if distance < 25.0 { 
                        println!("Attempting connection between {} and {}", start_node_id_val, end_node_comp.id);
                        connection_attempt_writer.send(ConnectionAttemptEvent {
                            node1_id: start_node_id_val,
                            node2_id: end_node_comp.id,
                        });
                        end_node_found = true;
                        break;
                    }
                }
                if end_node_found {
                     println!("Connection drawn (event sent).");
                } else {
                    println!("Connection attempt failed - no end node found on release.");
                }
                // Deselect the starting node after any attempt (successful or not)
                commands.entity(start_entity_val).remove::<ActivatedNode>();
            }
            // Reset drag state regardless
            drag_state.start_node_entity = None;
            drag_state.start_node_id = None;
        }
    }
}


pub fn check_connection_attempt_system(
    mut commands: Commands,
    mut connection_events: EventReader<ConnectionAttemptEvent>,
    mut player_attempt: ResMut<PlayerAttempt>,
    node_query: Query<(Entity, &Node)>, // Simplified: only need Entity and Node component
    existing_connections: Query<&Connection>,
) {
    for event in connection_events.read() {
        let (id1, id2) = if event.node1_id < event.node2_id {
            (event.node1_id, event.node2_id)
        } else {
            (event.node2_id, event.node1_id)
        };

        let already_drawn_by_player = player_attempt.drawn_connections.contains(&(id1, id2));
        
        // Check if a Connection component for this pair already exists
        let mut connection_component_exists = false;
        for conn_comp in existing_connections.iter() {
            if let (Ok((_, n1_comp)), Ok((_, n2_comp))) = (node_query.get(conn_comp.start_node_entity), node_query.get(conn_comp.end_node_entity)) {
                let (exist_id1, exist_id2) = if n1_comp.id < n2_comp.id {(n1_comp.id, n2_comp.id)} else {(n2_comp.id, n1_comp.id)};
                if exist_id1 == id1 && exist_id2 == id2 {
                    connection_component_exists = true;
                    break;
                }
            }
        }


        if !already_drawn_by_player && !connection_component_exists {
            player_attempt.drawn_connections.insert((id1, id2));
            println!("Player connections: {:?}", player_attempt.drawn_connections);

            let mut node_entities: [Option<Entity>; 2] = [None, None];
            for (entity, node_comp) in node_query.iter() {
                if node_comp.id == id1 { node_entities[0] = Some(entity); }
                else if node_comp.id == id2 { node_entities[1] = Some(entity); }
            }

            if let (Some(e1), Some(e2)) = (node_entities[0], node_entities[1]) {
                 commands.spawn((
                    Connection { start_node_entity: e1, end_node_entity: e2 },
                    // SpatialBundle::default(), // Not strictly needed if only drawn by gizmos from query
                 )).insert(Name::new(format!("ConnectionComp_{}-{}", id1, id2)));
                 println!("Connection component spawned for {}-{}", id1, id2);
            }
        } else {
            println!("Connection {}-{} already attempted or component exists.", id1, id2);
        }
    }
}

pub fn persistent_connection_render_system(
    connection_query: Query<&Connection>, // Query for Connection components
    node_transform_query: Query<(&Transform, &Node)>, // To get positions from node entities
    mut gizmos: Gizmos,
) {
    for connection in connection_query.iter() {
        if let (Ok((start_transform, _)), Ok((end_transform, _))) = (
            node_transform_query.get(connection.start_node_entity),
            node_transform_query.get(connection.end_node_entity)
        ) {
            gizmos.line_2d(
                start_transform.translation.truncate(),
                end_transform.translation.truncate(),
                Color::GREEN, 
            );
        }
    }
}
