//! # Menu Plugin
//!
//! Handles the main menu logic and UI.
//! Allows the player to start the game.

use bevy::prelude::*;
use crate::components::{MainMenuUI, StartButton, MainMenuButton, LevelCompleteUI, GameplayUI, LevelDisplay, InfoTextDisplay};
use crate::game_state::GameState;
use crate::resources::LevelManager;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems to run when entering the MainMenu state
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            // Systems to run when exiting the MainMenu state
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            // Systems to run when entering the LevelComplete state
            .add_systems(OnEnter(GameState::LevelComplete), setup_level_complete_screen)
            // Systems to run when exiting the LevelComplete state
            .add_systems(OnExit(GameState::LevelComplete), cleanup_level_complete_screen)
            // Systems to run when entering the GameOver state
            .add_systems(OnEnter(GameState::GameOver), setup_game_over_screen)
            // Systems to run when exiting the GameOver state
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over_screen)
             // Systems to run when entering the GameWon state
            .add_systems(OnEnter(GameState::GameWon), setup_game_won_screen)
            // Systems to run when exiting the GameWon state
            .add_systems(OnExit(GameState::GameWon), cleanup_game_won_screen)
            // System for handling button interactions in various menu-like states
            .add_systems(Update, (
                start_button_interaction.run_if(in_state(GameState::MainMenu)),
                menu_button_interaction.run_if(in_state(GameState::LevelComplete).or_else(in_state(GameState::GameOver)).or_else(in_state(GameState::GameWon))),
            ));
    }
}

/// Sets up the main menu UI.
fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "EchoNet",
                TextStyle {
                    font_size: 80.0,
                    color: Color::WHITE,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            }));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    StartButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start Game",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

/// Cleans up main menu UI elements when exiting the state.
fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles interaction with the start button.
fn start_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.35, 0.75, 0.35).into();
                border_color.0 = Color::WHITE;
                // Transition to the Gameplay state or LevelLoading state
                game_state.set(GameState::LevelLoading);
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

/// Sets up the level complete screen.
fn setup_level_complete_screen(
    mut commands: Commands,
    level_manager: Res<LevelManager>
) {
    let current_level_id = level_manager.levels.get(level_manager.current_level_index).map_or(0, |l| l.id);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.4, 0.2).into(),
                ..default()
            },
            LevelCompleteUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Level {} Complete!", current_level_id),
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            }));

            // Next Level Button (if there is a next level)
            if level_manager.current_level_index + 1 < level_manager.levels.len() {
                 parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(300.0),
                                height: Val::Px(65.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        },
                        // Re-use StartButton component logic for "Next Level" for simplicity
                        // Or create a new NextLevelButton component and system
                        crate::components::NextLevelButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Next Level",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ));
                    });
            } else {
                 parent.spawn(TextBundle::from_section(
                    "You've completed all levels!",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                }));
            }

            // Main Menu Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(300.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                             margin: UiRect::all(Val::Px(10.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    MainMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Main Menu",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

/// Cleans up level complete UI elements.
fn cleanup_level_complete_screen(mut commands: Commands, query: Query<Entity, With<LevelCompleteUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Sets up the game over screen.
fn setup_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgb(0.5, 0.2, 0.2).into(),
                ..default()
            },
            GameplayUI, // Re-use for cleanup, or make a GameOverUI component
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Game Over",
                TextStyle {
                    font_size: 80.0,
                    color: Color::WHITE,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            }));
            // TODO: Could add a "Retry Level" button here

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(300.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    MainMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Main Menu",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

/// Cleans up game over UI elements.
fn cleanup_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameplayUI>>) {
    // Assuming GameOver uses GameplayUI or a specific GameOverUI marker
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Sets up the game won screen.
fn setup_game_won_screen(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.5, 0.8).into(), // A triumphant color
                ..default()
            },
            LevelCompleteUI, // Re-use for cleanup or make a GameWonUI component
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Congratulations! You Won!",
                TextStyle {
                    font_size: 70.0,
                    color: Color::WHITE,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            }));

            parent.spawn(TextBundle::from_section(
                "You have successfully guided the EchoNet!",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            }));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(300.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    MainMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Main Menu",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

/// Cleans up game won UI elements.
fn cleanup_game_won_screen(mut commands: Commands, query: Query<Entity, With<LevelCompleteUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Handles interaction with the main menu button (on LevelComplete/GameOver/GameWon screens).
fn menu_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.35, 0.35, 0.35).into();
                 border_color.0 = Color::WHITE;
                game_state.set(GameState::MainMenu);
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
File created successfully.
