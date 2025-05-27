//! # Resources Module
//!
//! Defines global resources used by the EchoNet game.
//! Resources are unique, globally accessible data structures.

use bevy::prelude::*;
use std::collections::VecDeque;

/// Represents the definition of a single level.
#[derive(Debug, Clone, Default)]
pub struct Level {
    pub id: usize,
    pub name: String,
    /// List of nodes with their positions and initial states.
    /// (Position, InitialState, Option<PuzzleElement>)
    pub nodes: Vec<(Vec2, crate::components::NodeState, Option<crate::components::PuzzleElement>)>,
    /// Defines connections between nodes by their index in the `nodes` vector.
    /// (index_node_a, index_node_b)
    pub connections: Vec<(usize, usize)>,
    /// Index of the starting node for the echo in this level.
    pub start_node_index: usize,
    /// Index of the target node for the echo in this level.
    pub target_node_index: usize,
    /// Optional time limit for completing the level.
    pub time_limit: Option<f32>,
    /// Instructions specific to this level.
    pub instructions: String,
    /// Order in which nodes (identified by their `id` in `Node` component) must be activated.
    pub required_activation_sequence: Option<Vec<usize>>,
}

/// Manages the levels in the game.
#[derive(Resource, Debug)]
pub struct LevelManager {
    pub levels: Vec<Level>,
    pub current_level_index: usize,
}

impl Default for LevelManager {
    fn default() -> Self {
        let mut levels = Vec::new();

        // Level 1: Simple linear path
        levels.push(Level {
            id: 1,
            name: "The Basics".to_string(),
            nodes: vec![
                (Vec2::new(-200.0, 0.0), crate::components::NodeState::Start, None),
                (Vec2::new(0.0, 0.0), crate::components::NodeState::Idle, None),
                (Vec2::new(200.0, 0.0), crate::components::NodeState::Target, None),
            ],
            connections: vec![(0, 1), (1, 2)],
            start_node_index: 0,
            target_node_index: 2,
            time_limit: Some(30.0),
            instructions: "Activate nodes in sequence to guide the echo to the target.".to_string(),
            required_activation_sequence: Some(vec![0,1,2]), // Node IDs, not indices
        });

        // Level 2: Branching path, requires choice
        levels.push(Level {
            id: 2,
            name: "A Fork in the Road".to_string(),
            nodes: vec![
                (Vec2::new(-300.0, 0.0), crate::components::NodeState::Start, None), // 0
                (Vec2::new(-100.0, 0.0), crate::components::NodeState::Idle, None), // 1
                (Vec2::new(100.0, 100.0), crate::components::NodeState::Idle, None), // 2 (decoy)
                (Vec2::new(100.0, -100.0), crate::components::NodeState::Target, None), // 3
            ],
            connections: vec![(0, 1), (1, 2), (1, 3)],
            start_node_index: 0,
            target_node_index: 3,
            time_limit: Some(25.0),
            instructions: "Choose the correct path to the target node.".to_string(),
            required_activation_sequence: Some(vec![0,1,3]),
        });
        
        // Level 3: More complex path with required order
        levels.push(Level {
            id: 3,
            name: "Ordered Activation".to_string(),
            nodes: vec![
                (Vec2::new(-400.0, 0.0), crate::components::NodeState::Start, Some(crate::components::PuzzleElement { required_order: Some(0), activation_time_limit: None})), // Node ID 0
                (Vec2::new(-200.0, 100.0), crate::components::NodeState::Idle, Some(crate::components::PuzzleElement { required_order: Some(2), activation_time_limit: None})), // Node ID 1
                (Vec2::new(0.0, 0.0), crate::components::NodeState::Idle, Some(crate::components::PuzzleElement { required_order: Some(1), activation_time_limit: None})),    // Node ID 2
                (Vec2::new(200.0, 100.0), crate::components::NodeState::Idle, None), // Node ID 3
                (Vec2::new(400.0, 0.0), crate::components::NodeState::Target, None), // Node ID 4
            ],
            connections: vec![(0,2), (2,1), (1,3), (3,4)], // Connections by index
            start_node_index: 0,
            target_node_index: 4,
            time_limit: Some(45.0),
            instructions: "Activate nodes in the displayed numerical order (0 -> 1 -> 2).".to_string(),
            required_activation_sequence: Some(vec![0,2,1]), // Order refers to Node IDs
        });


        LevelManager {
            levels,
            current_level_index: 0,
        }
    }
}

impl LevelManager {
    pub fn get_current_level(&self) -> Option<&Level> {
        self.levels.get(self.current_level_index)
    }

    pub fn load_next_level(&mut self) -> bool {
        if self.current_level_index + 1 < self.levels.len() {
            self.current_level_index += 1;
            true
        } else {
            false // No more levels
        }
    }

    pub fn reset_current_level(&mut self) {
        // Potentially reload level data if it were mutable,
        // but for now, just ensures the index is valid.
        // Actual level reset (entity despawning, etc.) handled by systems.
        if self.current_level_index >= self.levels.len() {
            self.current_level_index = 0;
        }
    }
}

/// Resource to keep track of game time, especially for levels with time limits.
#[derive(Resource, Debug)]
pub struct GameTimer {
    pub timer: Timer,
    pub time_limit_seconds: Option<f32>,
}

impl Default for GameTimer {
    fn default() -> Self {
        GameTimer {
            timer: Timer::from_seconds(0.0, TimerMode::Once), // Default, will be set per level
            time_limit_seconds: None,
        }
    }
}

/// Resource to manage the sequence of activated nodes by the player.
#[derive(Resource, Debug, Default)]
pub struct PlayerActivationSequence {
    pub activated_node_ids: Vec<usize>, // Stores the ID of the Node component
}

impl PlayerActivationSequence {
    pub fn add_activated_node(&mut self, node_id: usize) {
        if !self.activated_node_ids.contains(&node_id) {
            self.activated_node_ids.push(node_id);
        }
    }

    pub fn clear(&mut self) {
        self.activated_node_ids.clear();
    }
}

/// Resource to store the path the echo should follow, determined by puzzle logic.
/// This path is a list of *Entities* corresponding to the Nodes.
#[derive(Resource, Debug, Default)]
pub struct EchoPath {
    pub path_node_entities: VecDeque<Entity>,
}

impl EchoPath {
    pub fn set_path(&mut self, path: Vec<Entity>) {
        self.path_node_entities = path.into();
    }

    pub fn get_next_node_in_path(&mut self) -> Option<Entity> {
        self.path_node_entities.pop_front()
    }
    
    pub fn is_empty(&self) -> bool {
        self.path_node_entities.is_empty()
    }

    pub fn clear(&mut self) {
        self.path_node_entities.clear();
    }
}
File created successfully.
