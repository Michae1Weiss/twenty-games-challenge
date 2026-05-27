use bevy::color::palettes::css::{BLACK, WHITE};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_hanabi::prelude::*;
use rand::prelude::*;

mod aim_line;
pub mod assets;
mod ball;
mod brick;
mod bullet_time;
mod collision;
mod input;
mod paddle;
mod pause;
mod phase;
mod respawn;
mod round;
mod rules;
mod score;
mod wall;

use crate::CANVAS_SIZE;
use crate::game::paddle::Paddle;
pub use crate::state::GameState;
use crate::ui::UiAssets;
pub use assets::TextureAssets;
pub use brick::Brick;
pub use collision::Collision;
pub use round::EndRound;
pub use round::RestartRound;
pub use round::StartRound;

pub(crate) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::AssetLoading).load_collection::<TextureAssets>(),
    )
    .insert_resource(ClearColor(Color::from(BLACK)))
    .add_plugins(HanabiPlugin)
    .add_plugins((
        pause::plugin,
        ball::plugin,
        brick::plugin,
        bullet_time::plugin,
        rules::plugin,
        round::plugin,
        collision::plugin,
        phase::plugin,
        aim_line::plugin,
        paddle::plugin,
        score::plugin,
    ))
    .add_systems(Startup, startup)
    .add_systems(OnEnter(GameState::Playing), spawn_background)
    .add_systems(OnEnter(GameState::GameOver), show_restart_text)
    .add_systems(OnEnter(GameState::Won), show_victory_text)
    .add_systems(
        Update,
        restart_game.run_if(
            (in_state(GameState::GameOver).or(in_state(GameState::Won)))
                .and(input_just_pressed(KeyCode::KeyR)),
        ),
    )
    .add_systems(
        FixedUpdate,
        paddle_controls.run_if(in_state(GameState::Playing)),
    );
}

const PADDLE_SPEED: f32 = 600.;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component, Default, Debug)]
enum PaddleMovement {
    Left,
    Right,
    #[default]
    Idle,
}

#[derive(Component)]
struct HalfSize(Vec2);

// TODO: create a plugin for background
fn startup(mut commands: Commands) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(CANVAS_SIZE.x + 4.0, CANVAS_SIZE.y + 4.0)),
            color: Color::from(BLACK),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -3.0),
    ));
}

fn spawn_background(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn((
        Sprite {
            image: texture_assets.background.clone(),
            custom_size: Some(CANVAS_SIZE),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -2.0),
        DespawnOnExit(GameState::Playing),
    ));
}

// TODO: move somewhere else or reimplement but in more bevy-ish way
fn restart_game(
    mut next_state: ResMut<NextState<GameState>>,
    mut start_round: MessageWriter<RestartRound>,
) {
    start_round.write(RestartRound);
    next_state.set(GameState::Playing);
}

fn show_restart_text(mut commands: Commands, ui_assets: Res<UiAssets>) {
    let mut rng = rand::rng();

    let insults = [
        "You suck",
        "Nice try, loser",
        "Skill issue",
        "Embarrassing",
        "That was terrible",
        "Delete the game",
        "Disappointing",
        "Noob",
        "Absolute garbage",
        "My grandma plays better",
        "Try using your hands next time",
    ];

    commands.spawn((
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: percent(100.),
            height: percent(100.),
            ..default()
        },
        DespawnOnExit(GameState::GameOver),
        children![
            Text::new(format!(
                "{}! Press R to Restart Game",
                insults.choose(&mut rng).unwrap()
            )),
            TextFont {
                font: ui_assets.font.clone(),
                font_size: 67.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ],
    ));
}

fn show_victory_text(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: percent(100.),
            height: percent(100.),
            ..default()
        },
        DespawnOnExit(GameState::Won),
        children![
            Text::new("Victory! Press R to Restart Game"),
            TextFont {
                font: ui_assets.font.clone(),
                font_size: 67.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ],
    ));
}

fn paddle_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut paddles: Query<(&mut Transform, &HalfSize, &mut PaddleMovement), With<Paddle>>,
    time: Res<Time>,
) {
    for (mut transform, half_size, mut paddle_movement) in &mut paddles {
        if keys.pressed(KeyCode::KeyA)
            && transform.translation.x - half_size.0.x > -CANVAS_SIZE.x / 2.
        {
            *paddle_movement = PaddleMovement::Left;
            transform.translation.x -= PADDLE_SPEED * time.delta_secs();
        } else if keys.pressed(KeyCode::KeyD)
            && transform.translation.x + half_size.0.x < CANVAS_SIZE.x / 2.
        {
            *paddle_movement = PaddleMovement::Right;
            transform.translation.x += PADDLE_SPEED * time.delta_secs();
        } else {
            *paddle_movement = PaddleMovement::Idle;
        }
    }
}
