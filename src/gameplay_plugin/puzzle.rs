use bevy::prelude::*; // Added
use crate::components::{Node, GameplayUI};
use crate::resources::{CurrentLevel, PuzzleSpec, PlayerAttempt, GameFont};
use crate::game_state::GameState;
use super::PuzzleCompleteEvent; 
// use std::collections::HashSet; // Removed as unused

const MAX_LEVELS: usize = 2;
fn get_level_spec(level_id: usize) -> PuzzleSpec {
    match level_id {
        0 => PuzzleSpec { 
            node_positions: vec![
                Vec2::new(-150.0, 0.0), Vec2::new(0.0, 100.0), Vec2::new(150.0, 0.0)
            ],
            correct_connections: [(0,1), (1,2)].iter().cloned().collect(),
        },
        1 => PuzzleSpec { 
            node_positions: vec![
                Vec2::new(-200.0, 100.0), Vec2::new(-200.0, -100.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(200.0, 100.0), Vec2::new(200.0, -100.0),
            ],
            correct_connections: [(0,2), (1,2), (2,3), (2,4)].iter().cloned().collect(),
        },
        _ => get_level_spec(0), 
    }
}

pub fn setup_level_system(
    mut commands: Commands,
    mut current_level: ResMut<CurrentLevel>,
    mut puzzle_spec: ResMut<PuzzleSpec>,
    mut player_attempt: ResMut<PlayerAttempt>,
    mut next_game_state: ResMut<NextState<GameState>>,
    game_font: Res<GameFont>, 
) {
    current_level.total_levels = MAX_LEVELS;
    
    if current_level.level_id >= MAX_LEVELS {
        current_level.level_id = 0;
    }
    
    *puzzle_spec = get_level_spec(current_level.level_id);
    player_attempt.drawn_connections.clear(); 

    commands.spawn((Camera2dBundle::default(), GameplayUI)); 

    for (idx, pos) in puzzle_spec.node_positions.iter().enumerate() {
        let node_color = Color::srgb(0.2, 0.2, 0.8); // Corrected Color
        commands.spawn((
            SpriteBundle { 
                sprite: Sprite {
                    color: node_color,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },
            Node { id: idx, original_color: node_color },
            Name::new(format!("Node_{}", idx)),
            GameplayUI, 
        ));
    }
    
     commands.spawn((
        TextBundle::from_section(
            format!("Level: {}/{}", current_level.level_id + 1, current_level.total_levels),
            TextStyle { 
                font: game_font.0.clone(),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style { 
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        GameplayUI
    ));

    println!("Setting up Level: {}", current_level.level_id);
    next_game_state.set(GameState::Playing);
}

pub fn check_puzzle_completion_system(
    puzzle_spec: Res<PuzzleSpec>,
    player_attempt: Res<PlayerAttempt>,
    mut puzzle_complete_event: EventWriter<PuzzleCompleteEvent>,
    mut already_fired_event: Local<bool>, 
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() != GameState::Playing { 
        *already_fired_event = false; 
        return;
    }

    if !*already_fired_event && 
       player_attempt.drawn_connections.len() == puzzle_spec.correct_connections.len() &&
       player_attempt.drawn_connections.is_subset(&puzzle_spec.correct_connections) {
        println!("Puzzle Complete!");
        puzzle_complete_event.write(PuzzleCompleteEvent); // Corrected deprecated send
        *already_fired_event = true;
    }
}
