use bevy::prelude::*; // Added
use crate::game_state::GameState;
use crate::resources::{CurrentLevel, GameFont};
use crate::components::{LevelCompleteUI, GameButtonAction};
use crate::gameplay_plugin::PuzzleCompleteEvent;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LevelComplete), setup_level_complete_ui)
            .add_systems(Update, 
                (level_complete_button_interaction_system).run_if(in_state(GameState::LevelComplete))
            )
            .add_systems(OnExit(GameState::LevelComplete), cleanup_level_complete_ui)
            .add_systems(Update, handle_puzzle_complete_event);
    }
}

fn handle_puzzle_complete_event(
    mut puzzle_complete_reader: EventReader<PuzzleCompleteEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() == GameState::Playing {
        if puzzle_complete_reader.read().next().is_some() {
            next_game_state.set(GameState::LevelComplete);
        }
    }
}

fn setup_level_complete_ui(
    mut commands: Commands, 
    game_font: Res<GameFont>, 
    current_level: Res<CurrentLevel>
) {
    commands.spawn((Camera2dBundle::default(), LevelCompleteUI)); 

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.85).into(), // Corrected
            ..default()
        },
        LevelCompleteUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("Level {} Complete!", current_level.level_id + 1),
            TextStyle {
                font: game_font.0.clone(),
                font_size: 60.0,
                color: Color::srgb(0.5, 1.0, 0.5), // Corrected
            },
        ).with_style(Style { margin: UiRect::bottom(Val::Px(30.0)), ..default() }));

        if current_level.level_id < current_level.total_levels - 1 {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: BorderColor(Color::srgb(0.3, 0.3, 0.7)), // Corrected
                    background_color: Color::srgb(0.2, 0.2, 0.6).into(), // Corrected
                    ..default()
                },
                GameButtonAction::NextLevel,
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Next Level",
                    TextStyle {
                        font: game_font.0.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ));
            });
        } else {
             parent.spawn(TextBundle::from_section(
                "All Levels Cleared!",
                TextStyle {
                    font: game_font.0.clone(),
                    font_size: 40.0,
                    color: Color::srgb(0.6, 1.0, 0.6), // Corrected
                },
            ).with_style(Style { margin: UiRect::bottom(Val::Px(20.0)), ..default() }));
        }

        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                     border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                border_color: BorderColor(Color::srgb(0.7, 0.3, 0.3)), // Corrected
                background_color: Color::srgb(0.6, 0.2, 0.2).into(), // Corrected
                ..default()
            },
            GameButtonAction::BackToMenu,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Main Menu",
                TextStyle {
                    font: game_font.0.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

fn level_complete_button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &GameButtonAction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut current_level: ResMut<CurrentLevel>,
) {
    for (interaction, button_action, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button_action {
                    GameButtonAction::NextLevel => {
                        if current_level.level_id < current_level.total_levels - 1 {
                            current_level.level_id += 1;
                            next_game_state.set(GameState::LoadingLevel);
                        }
                    }
                    GameButtonAction::BackToMenu => {
                        next_game_state.set(GameState::MainMenu);
                    }
                    _ => {} 
                }
            }
            Interaction::Hovered => {
                match button_action {
                    GameButtonAction::NextLevel => {
                        *bg_color = Color::srgb(0.3, 0.3, 0.7).into(); // Corrected
                        *border_color = BorderColor(Color::WHITE);
                    }
                    GameButtonAction::BackToMenu => {
                         *bg_color = Color::srgb(0.7, 0.3, 0.3).into(); // Corrected
                         *border_color = BorderColor(Color::WHITE);
                    }
                    _ => {}
                }
            }
            Interaction::None => {
                 match button_action {
                    GameButtonAction::NextLevel => {
                        *bg_color = Color::srgb(0.2, 0.2, 0.6).into(); // Corrected
                        *border_color = BorderColor(Color::srgb(0.3, 0.3, 0.7)); // Corrected
                    }
                    GameButtonAction::BackToMenu => {
                         *bg_color = Color::srgb(0.6, 0.2, 0.2).into(); // Corrected
                        *border_color = BorderColor(Color::srgb(0.7, 0.3, 0.3)); // Corrected
                    }
                     _ => {}
                }
            }
        }
    }
}

fn cleanup_level_complete_ui(mut commands: Commands, query: Query<Entity, With<LevelCompleteUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn(); // Corrected
    }
}
