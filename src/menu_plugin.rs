use bevy::prelude::*;
use bevy::app::AppExit;
use crate::game_state::GameState;
use crate::components::{MainMenuUI, MenuButtonAction};
use crate::resources::GameFont;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, 
                (menu_button_interaction_system).run_if(in_state(GameState::MainMenu))
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
    }
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load font
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.insert_resource(GameFont(font.clone()));

    commands.spawn((Camera2dBundle::default(), MainMenuUI));

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
            ..default()
        },
        MainMenuUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "EchoNet",
            TextStyle {
                font: font.clone(),
                font_size: 80.0,
                color: Color::WHITE,
            },
        ).with_style(Style { margin: UiRect::bottom(Val::Px(50.0)), ..default() }));

        // Play Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            MenuButtonAction::Play,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });

        // Quit Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            MenuButtonAction::Quit,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Quit",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    });
}

fn menu_button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.35, 0.75, 0.35).into(); // Pressed color
                match menu_button_action {
                    MenuButtonAction::Play => {
                        next_game_state.set(GameState::LoadingLevel);
                    }
                    MenuButtonAction::Quit => {
                        app_exit_events.send(AppExit);
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into(); // Hover color
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into(); // Normal color
            }
        }
    }
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
