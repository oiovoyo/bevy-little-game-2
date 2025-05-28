use bevy::prelude::*; 
use crate::game_state::GameState;
use crate::resources::{CurrentLevel, PlayerAttempt, PuzzleSpec};
use crate::components::GameplayUI;

pub mod node;
pub mod connection;
pub mod puzzle;
pub mod echo;

#[derive(Event, Debug)]
pub struct ConnectionAttemptEvent {
    pub node1_id: usize,
    pub node2_id: usize,
}

#[derive(Event, Debug)]
pub struct PuzzleCompleteEvent;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ConnectionAttemptEvent>()
            .add_event::<PuzzleCompleteEvent>()
            .init_resource::<CurrentLevel>()
            .init_resource::<PlayerAttempt>()
            .init_resource::<PuzzleSpec>() 
            .add_systems(OnEnter(GameState::LoadingLevel), puzzle::setup_level_system)
            .add_systems(Update, 
                (
                    node::node_interaction_system,
                    echo::echo_visualization_system, 
                    connection::draw_connection_system,
                    connection::check_connection_attempt_system,
                    connection::persistent_connection_render_system, 
                    puzzle::check_puzzle_completion_system,
                    gameplay_keyboard_input_system,
                ).run_if(in_state(GameState::Playing))
            )
            .add_systems(OnExit(GameState::Playing), cleanup_gameplay_entities);
    }
}

fn gameplay_keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    current_level: Res<CurrentLevel>,
     mut puzzle_complete_event: EventWriter<PuzzleCompleteEvent>, 
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        next_game_state.set(GameState::MainMenu);
    }
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_game_state.set(GameState::LoadingLevel);
    }
    if keyboard_input.just_pressed(KeyCode::KeyN) && current_level.level_id < current_level.total_levels -1 {
         next_game_state.set(GameState::LevelComplete); 
    }
    if keyboard_input.just_pressed(KeyCode::Space) { 
        puzzle_complete_event.write(PuzzleCompleteEvent); // Corrected
    }
}

fn cleanup_gameplay_entities(
    mut commands: Commands, 
    node_query: Query<Entity, With<crate::components::Node>>,
    connection_query: Query<Entity, With<crate::components::Connection>>,
    gameplay_ui_query: Query<Entity, With<GameplayUI>>, 
) {
    for entity in node_query.iter() {
        commands.entity(entity).despawn(); // Corrected
    }
    for entity in connection_query.iter() {
        commands.entity(entity).despawn(); // Corrected
    }
     for entity in gameplay_ui_query.iter() { 
        commands.entity(entity).despawn(); // Corrected
    }
}
