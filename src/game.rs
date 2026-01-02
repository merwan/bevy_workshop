use bevy::prelude::*;

use crate::{GameAssets, GameState};

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), display_level)
        .add_systems(
            FixedUpdate,
            (
                control_player.run_if(in_state(GameState::Game)),
                move_player.run_if(in_state(GameState::Game)),
            ),
        );
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerVelocity {
    value: f32,
}

#[derive(Component)]
struct Asteroid;

fn display_level(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_image(game_assets.player_ship.clone()),
        Player,
        PlayerVelocity { value: 0.0 },
        StateScoped(GameState::Game),
        children![(
            Sprite::from_image(game_assets.player_engine.clone()),
            Transform::from_xyz(0.0, -40.0, 0.0),
            Visibility::Hidden,
        )],
    ));

    commands.spawn((
        Sprite::from_image(game_assets.asteroid.clone()),
        Transform::from_xyz(300.0, -200.0, 0.0),
        Asteroid,
        StateScoped(GameState::Game),
    ));
}

fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut PlayerVelocity, &mut Transform, &Children), With<Player>>,
    mut visibility: Query<&mut Visibility>,
    time: Res<Time>,
) -> Result {
    let (mut player_velocity, mut player_transform, children) = player.into_inner();

    let fixed_rotation_rate = 0.2;
    let rotation_rate = fixed_rotation_rate * 60.0 * time.delta_secs();

    if keyboard_input.pressed(KeyCode::KeyA) {
        player_transform.rotate_z(rotation_rate);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player_transform.rotate_z(-rotation_rate);
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        player_velocity.value += 0.2;
        visibility
            .get_mut(children[0])?
            .set_if_neq(Visibility::Visible);
    } else {
        // player_velocity.value = 0.0;
        visibility
            .get_mut(children[0])?
            .set_if_neq(Visibility::Hidden);
    }
    Ok(())
}
fn move_player(player: Single<(&PlayerVelocity, &mut Transform), With<Player>>) -> Result {
    let (player_velocity, mut player_transform) = player.into_inner();
    let forward = player_transform.local_y();
    player_transform.translation += forward * player_velocity.value;
    Ok(())
}
