use bevy::prelude::*;

use crate::GameState;

pub fn start_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::StartMenu), display_start_screen)
        .add_systems(
            Update,
            switch_to_play.run_if(in_state(GameState::StartMenu)),
        );
}

fn display_start_screen(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![(
            Text::new("Press any key to start..."),
            TextFont {
                font_size: 100.0,
                ..default()
            }
        )],
        StateScoped(GameState::StartMenu),
    ));
}

fn switch_to_play(mut next: ResMut<NextState<GameState>>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.get_just_pressed().next().is_some() {
        next.set(GameState::Game);
    }
}
