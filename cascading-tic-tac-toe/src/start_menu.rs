use bevy::{app::AppExit, prelude::*};
use crate::{GameState, MenuState, PlayingState};
use crate::ui_components::bundles::{button_bundle, image_bundle, text_bundle};
use crate::utils::despawn_screen::despawn_screen;

//colors
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

struct ButtonParams {
    text: &'static str,
    text_color: Color,
    icon_path: &'static str,
    action: MenuButtonAction,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin{
   fn build(&self, app: &mut App){
        app
            // Systems to handle the main menu screen
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            // Systems to despawn main menu
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            // Common systems to all screens that handles buttons behavior
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(PlayingState::NotPlaying)),
            );
    }
}
fn main_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let buttons = vec![
        ButtonParams {
            text: "New Game",
            text_color: TEXT_COLOR,
            icon_path: "texture/icons/right-arrow.png",
            action: MenuButtonAction::Play,
        },
        ButtonParams {
            text: "!Settings!",
            text_color: Color::rgb(244.0, 0.0, 9.0),
            icon_path: "texture/icons/wrench.png",
            action: MenuButtonAction::Settings,
        },
        ButtonParams {
            text: "Quit",
            text_color: TEXT_COLOR,
            icon_path: "texture/icons/exit.png",
            action: MenuButtonAction::Quit,
        },
    ];

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(text_bundle("Cascading Tic-Tac-Toe", &asset_server, (80.0, TEXT_COLOR))
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    // Display three buttons for each action available from the main menu:
                    for params in buttons {
                        parent
                            .spawn((
                                button_bundle(
                                    (Val::Px(250.0), Val::Px(65.0), Option::from(UiRect::all(Val::Px(20.0))), JustifyContent::Center, AlignItems::Center),
                                    NORMAL_BUTTON.into()
                                ),
                                params.action,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load(params.icon_path);
                                parent.spawn(image_bundle(UiImage::new(icon)));
                                parent.spawn(text_bundle(params.text, &asset_server, (40.0, params.text_color)));
                            });
                    }
                });
        });
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut playing_state: ResMut<NextState<PlayingState>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::GameOngoing);
                    playing_state.set(PlayingState::Local);
                    menu_state.set(MenuState::Disabled);
                }
                //setting button interaction disable currently!!!
                MenuButtonAction::Settings => menu_state.set(MenuState::Main),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::Main);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::Main);
                }
                //setting button interaction disable currently!!!
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}
