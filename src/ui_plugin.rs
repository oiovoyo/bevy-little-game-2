//! # UI Plugin
//!
//! Handles the display of gameplay UI elements like instructions, level info, and timer.

use bevy::prelude::*;
use crate::components::{GameplayUI, LevelDisplay, InfoTextDisplay, TimerTextDisplay, NextLevelButton, MainMenuButton};
use crate::game_state::GameState;
use crate::resources::{LevelManager, GameTimer};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems to run when entering the Gameplay state (to set up UI)
            .add_systems(OnEnter(GameState::Gameplay), setup_gameplay_ui)
            // Systems to run while in Gameplay state (to update UI)
            .add_systems(Update, (
                update_level_display.run_if(in_state(GameState::Gameplay)),
                update_timer_display.run_if(in_state(GameState::Gameplay)),
                update_instructions_display.run_if(in_state(GameState::Gameplay)),
                next_level_button_interaction.run_if(in_state(GameState::LevelComplete)),
                // MainMenuButton interaction is handled in menu_plugin
            ))
            // Systems to run when exiting the Gameplay state (to clean up UI)
            .add_systems(OnExit(GameState::Gameplay), cleanup_gameplay_ui);
    }
}

/// Sets up the gameplay UI elements.
fn setup_gameplay_ui(mut commands: Commands, level_manager: Res<LevelManager>) {
    let current_level = level_manager.get_current_level().expect("Level should be loaded");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween, // Pushes top and bottom sections apart
                    ..default()
                },
                ..default()
            },
            GameplayUI,
        ))
        .with_children(|parent| {
            // Top section for level info and timer
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::FlexStart, // Align items to the top
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.3).into(),
                    ..default()
                })
                .with_children(|top_bar| {
                    top_bar.spawn((
                        TextBundle::from_section(
                            format!("Level: {}", current_level.id),
                            TextStyle {
                                font_size: 30.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        LevelDisplay,
                    ));
                    top_bar.spawn((
                        TextBundle::from_section(
                            "Timer: --:--",
                            TextStyle {
                                font_size: 30.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        TimerTextDisplay,
                    ));
                });

            // Bottom section for instructions
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd, // Align items to the bottom
                        ..default()
                    },
                     background_color: Color::rgba(0.0, 0.0, 0.0, 0.3).into(),
                    ..default()
                })
                .with_children(|bottom_bar| {
                    bottom_bar.spawn((
                        TextBundle::from_section(
                            current_level.instructions.clone(),
                            TextStyle {
                                font_size: 24.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ).with_text_justify(JustifyText::Center),
                        InfoTextDisplay,
                    ));
                });
        });
}

/// Updates the level display text.
fn update_level_display(
    level_manager: Res<LevelManager>,
    mut query: Query<&mut Text, With<LevelDisplay>>,
) {
    if level_manager.is_changed() {
        if let Some(current_level) = level_manager.get_current_level() {
            for mut text in query.iter_mut() {
                text.sections[0].value = format!("Level: {}", current_level.id);
            }
        }
    }
}

/// Updates the timer display.
fn update_timer_display(game_timer: Res<GameTimer>, mut query: Query<&mut Text, With<TimerTextDisplay>>) {
    if game_timer.is_changed() || game_timer.timer.just_finished() { // Update on change or when timer just finished (to show 0)
        for mut text in query.iter_mut() {
            if let Some(limit) = game_timer.time_limit_seconds {
                let remaining = (limit - game_timer.timer.elapsed_secs()).max(0.0);
                let minutes = (remaining / 60.0).floor() as u32;
                let seconds = (remaining % 60.0).floor() as u32;
                text.sections[0].value = format!("Timer: {:02}:{:02}", minutes, seconds);
            } else {
                text.sections[0].value = "Timer: N/A".to_string();
            }
        }
    }
}

/// Updates the instructions display text if the level changes.
fn update_instructions_display(
    level_manager: Res<LevelManager>,
    mut query: Query<&mut Text, With<InfoTextDisplay>>,
) {
    if level_manager.is_changed() { // Only update if the level manager resource itself has changed
        if let Some(current_level) = level_manager.get_current_level() {
            for mut text in query.iter_mut() {
                text.sections[0].value = current_level.instructions.clone();
            }
        }
    }
}


/// Cleans up gameplay UI elements when exiting the state.
fn cleanup_gameplay_ui(mut commands: Commands, query: Query<Entity, With<GameplayUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Handles interaction with the next level button on the LevelComplete screen.
fn next_level_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<NextLevelButton>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut level_manager: ResMut<LevelManager>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.35, 0.75, 0.35).into();
                border_color.0 = Color::WHITE;
                if level_manager.load_next_level() {
                    game_state.set(GameState::LevelLoading);
                } else {
                    // This case should ideally be handled by not showing the button,
                    // but as a fallback, go to GameWon state.
                    game_state.set(GameState::GameWon);
                }
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
