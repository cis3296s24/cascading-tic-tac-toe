use bevy::prelude::*;

use crate::{GameState, PlayerTurn, PlayingState, StateWrapper, UiTheme};

#[derive(Component)]
struct ReloadButton;

pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Local), setup_restart_button.system())
           .add_systems(Update, reload_button_interactions.system())
           .add_systems(OnEnter(PlayingState::NotPlaying), new_game.system());
    }
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(7.0),
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexEnd,
            padding: UiRect {
                left: Val::Px(0.),
                right: Val::Px(20.),
                top: Val::Px(20.),
                bottom: Val::Px(0.),
            },
            ..Default::default()
        },
        background_color: Color::NONE.into(),
        ..Default::default()
    }
}

pub fn button(theme: &Res<UiTheme>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            padding: UiRect::all(Val::Px(5.)),
            width: Val::Auto,
            height: Val::Auto,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: theme.button,
        ..Default::default()
    }
}

pub fn button_text(
    asset_server: &Res<AssetServer>,
    theme: &Res<UiTheme>,
    label: &str,
) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: theme.button_text,
            },
        ),
        ..Default::default()
    }
}

fn setup_restart_button(
    mut commands: Commands,
    theme: Res<UiTheme>,
    asset_server: Res<AssetServer>,
) {
    // Spawn the root node for the restart button
    commands.spawn(root()).with_children(|parent| {
        // Spawn the button node
        parent
            .spawn(button(&theme))
            .with_children(|parent| {
                // Spawn the text node for the button
                parent.spawn(button_text(&asset_server, &theme, "New Game"));
            })
            // Insert the ReloadButton component to mark it as a reload button
            .insert(ReloadButton);
    });
}

fn reload_button_interactions(
    theme: Res<UiTheme>,
    mut buttons: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>, With<ReloadButton>)>,
    mut game_next_state: ResMut<NextState<PlayingState>>,
) {
    // Iterate through all the reload buttons and handle their interactions
    for (interaction, mut color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Change the button color when pressed
                *color = theme.button;
                // Set the next game state to NotPlaying when the button is pressed
                game_next_state.set(PlayingState::NotPlaying);
            }
            Interaction::Hovered => *color = theme.button_hovered,
            Interaction::None => *color = theme.button,
        }
    }
}

fn new_game(
    commands: Commands,
    query: Query<Entity>,
    playing_state: ResMut<State<PlayingState>>,
    playing_next_state: ResMut<NextState<PlayingState>>,
    game_state: ResMut<State<GameState>>,
    game_next_state: ResMut<NextState<GameState>>,
    player_turn_state: ResMut<State<PlayerTurn>>,
    player_turn_next_state: ResMut<NextState<PlayerTurn>>,
) {
    // Reload the game state
    reload_game(
        commands,
        query,
        StateWrapper {
            current: playing_state.clone(),
            next: playing_next_state,
        },
        StateWrapper {
            current: game_state.clone(),
            next: game_next_state,
        },
       StateWrapper {
           current: player_turn_state.clone(),
           next: player_turn_next_state,
        }
    );
}

fn reload_game(
    mut commands: Commands,
    query: Query<Entity>,
    mut playing_state: StateWrapper<PlayingState>,
    mut game_state: StateWrapper<GameState>,
    mut player_turn_state: StateWrapper<PlayerTurn>,
) {
    // Despawn all entities related to the game
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    // Set the playing state to Local
    playing_state.next.set(PlayingState::Local);

    // Set the game state to GameOngoing if it's not already
    if game_state.current != GameState::GameOngoing {
        game_state.next.set(GameState::GameOngoing);
    }

    // Set the player turn state to X if it's not already
    if player_turn_state.current != PlayerTurn::X {
        player_turn_state.next.set(PlayerTurn::X);
    }
}