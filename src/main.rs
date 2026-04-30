use bevy::{
    camera::ScalingMode,
    color::palettes::{
        css::{BLACK, WHITE},
        tailwind::{SKY_50, SKY_300, SKY_500, SKY_800, SLATE_900},
    },
    prelude::*,
};

const BALL_SIZE: f32 = 10.;
const CANVAS_SIZE: Vec2 = Vec2::new(1280., 720.);
const BRICK_SIZE: Vec2 = Vec2::new(80., 40.);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Wall(Plane2d);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::from(SKY_300)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, ball_movement)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: CANVAS_SIZE.x + BRICK_SIZE.x,
                min_height: CANVAS_SIZE.y + BRICK_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Ball,
        Velocity(Vec2::new(-20., -40.)),
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::from(SLATE_900))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        children![(
            Mesh2d(meshes.add(Circle::new(BALL_SIZE - 1.0))),
            MeshMaterial2d(materials.add(Color::from(WHITE))),
            Transform::from_xyz(0.0, 0.0, 1.0)
        )],
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(CANVAS_SIZE.x + 4.0, CANVAS_SIZE.y + 4.0)),
            color: Color::from(SKY_50),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -3.0),
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(CANVAS_SIZE),
            color: Color::from(SKY_800),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -2.0),
    ));

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
        Transform::from_xyz(-CANVAS_SIZE.y / 2.0, 0.0, 0.0),
    ));
    // Top wall
    commands.spawn((
        Wall(Plane2d::new(Vec2::NEG_Y)),
        Transform::from_xyz(CANVAS_SIZE.y / 2.0, 0.0, 0.0),
    ));
}

fn ball_movement(mut query: Query<(&Velocity, &mut Transform), With<Ball>>, time: Res<Time>) {
    for (velocity, mut transform) in &mut query {
        let ball_movement_this_frame = velocity.0 * time.delta_secs();
        transform.translation += ball_movement_this_frame.extend(0.0);
    }
}
