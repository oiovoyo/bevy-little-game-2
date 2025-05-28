use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct CurrentLevel {
    pub level_id: usize,
    pub total_levels: usize, // To know when we are at the last level
}

#[derive(Resource)]
pub struct PuzzleSpec {
    pub node_positions: Vec<Vec2>,
    // Tuples of (node_id_1, node_id_2) representing correct connections
    pub correct_connections: HashSet<(usize, usize)>, 
}

impl Default for PuzzleSpec {
    fn default() -> Self {
        // Default to an empty puzzle or a very simple first level
        PuzzleSpec {
            node_positions: vec![Vec2::new(-100.0, 0.0), Vec2::new(100.0, 0.0)],
            correct_connections: [(0,1)].iter().cloned().collect(),
        }
    }
}

#[derive(Resource, Default)]
pub struct PlayerAttempt {
    // Tuples of (node_id_1, node_id_2) representing player drawn connections
    pub drawn_connections: HashSet<(usize, usize)>,
}

#[derive(Resource)]
pub struct GameFont(pub Handle<Font>); // To store the loaded font handle

#[derive(Resource, Default)]
pub struct LevelManager {
    pub current_level: usize,
    pub total_levels: usize,
}

#[derive(Resource)]
pub struct GameTimer {
    pub timer: Timer,
}

impl Default for GameTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}
