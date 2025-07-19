use bevy::prelude::*;

mod splash;
mod start;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Workshop".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_plugins(splash::splash_plugin)
        .add_plugins(start::start_plugin)
        .run();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Splash,
    StartMenu,
    Play,
}
