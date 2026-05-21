use bevy::color::palettes::css::{BLACK, DARK_GRAY, WHITE, WHITE_SMOKE};
use bevy::prelude::*;
use bevy::{
    color::palettes::tailwind::{SKY_50, SKY_300, SKY_800, SLATE_50},
    input::common_conditions::input_just_pressed,
};
use bevy_asset_loader::prelude::*;
use bevy_hanabi::prelude::*;

pub mod assets;
mod ball;
mod input;
mod pause;
mod physics;

use crate::CANVAS_SIZE;
use crate::{game::assets::TextureAssets, state::GameState};

pub(crate) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::AssetLoading).load_collection::<TextureAssets>(),
    )
    .insert_resource(ClearColor(Color::from(BLACK)))
    .add_plugins(HanabiPlugin)
    .add_plugins(pause::plugin)
    .add_plugins(ball::plugin)
    .add_systems(Startup, startup)
    .add_systems(
        OnEnter(GameState::Playing),
        (spawn_background, spawn_new_game),
    )
    .add_systems(OnEnter(GameState::GameOver), show_restart_text)
    .add_systems(
        Update,
        restart_game.run_if(in_state(GameState::GameOver).and(input_just_pressed(KeyCode::KeyR))),
    )
    .add_systems(
        FixedUpdate,
        (paddle_controls,).run_if(in_state(GameState::Playing)),
    );
}

const BRICK_SIZE: Vec2 = Vec2::new(80., 40.);
const DEFAULT_PADDLE_SIZE: Vec2 = Vec2::new(200., 25.);
const PADDLE_SPEED: f32 = 600.;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Wall(Plane2d);

#[derive(Component)]
struct Paddle;

#[derive(Component, Default, Debug)]
enum PaddleMovement {
    Left,
    Right,
    #[default]
    Idle,
}

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct HalfSize(Vec2);

fn startup(mut commands: Commands) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(CANVAS_SIZE.x + 4.0, CANVAS_SIZE.y + 4.0)),
            color: Color::from(DARK_GRAY),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -3.0),
    ));

    // commands.spawn((
    //     Sprite {
    //         // image: texture_assets.background.clone(),
    //         custom_size: Some(CANVAS_SIZE),
    //         // color: Color::from(SKY_800),
    //         ..default()
    //     },
    //     Transform::from_xyz(0.0, 0.0, -2.0),
    // ));

    // Left wall
    commands.spawn((
        Wall(Plane2d::new(Vec2::X)),
        Transform::from_xyz(-CANVAS_SIZE.x / 2.0, 0.0, 0.0),
    ));
    // Right wall
    commands.spawn((
        Wall(Plane2d::new(Vec2::NEG_X)),
        Transform::from_xyz(CANVAS_SIZE.x / 2.0, 0.0, 0.0),
    ));
    // Bottom wall
    commands.spawn((
        Wall(Plane2d::new(Vec2::Y)),
        Transform::from_xyz(0.0, -CANVAS_SIZE.y / 2.0, 0.0),
    ));
    // Top wall
    commands.spawn((
        Wall(Plane2d::new(Vec2::NEG_Y)),
        Transform::from_xyz(0.0, CANVAS_SIZE.y / 2.0, 0.0),
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

fn spawn_new_game(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn((
        Sprite {
            image: texture_assets.paddle.clone(),
            custom_size: Some(DEFAULT_PADDLE_SIZE),
            ..default()
        },
        Transform::from_xyz(0.0, -CANVAS_SIZE.y * 3.0 / 8.0, 0.0),
        Paddle,
        PaddleMovement::default(),
        HalfSize(DEFAULT_PADDLE_SIZE / 2.),
        DespawnOnExit(GameState::Playing),
    ));

    let n_rows: i32 = 6;
    let n_columns: i32 = 13;

    for row in 0..n_rows {
        for column in 0..n_columns {
            commands.spawn((
                Brick,
                Sprite {
                    image: texture_assets.brick.clone(),
                    custom_size: Some(BRICK_SIZE),
                    ..default()
                },
                Transform::from_xyz(
                    -480. + BRICK_SIZE.x * column as f32,
                    240. - BRICK_SIZE.y * row as f32,
                    0.0,
                ),
                HalfSize(BRICK_SIZE / 2.),
                DespawnOnExit(GameState::Playing),
            ));
        }
    }
}

fn restart_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn show_restart_text(mut commands: Commands) {
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
            Text::new("Press R to Restart Game"),
            TextFont::from_font_size(67.0),
            TextColor(SLATE_50.into()),
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
