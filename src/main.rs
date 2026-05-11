use std::f32::consts::{FRAC_PI_4, PI};

use bevy::{
    camera::ScalingMode,
    color::palettes::{
        css::WHITE,
        tailwind::{SKY_50, SKY_300, SKY_800, SLATE_900},
    },
    math::{
        FloatOrd,
        bounding::{Aabb2d, RayCast2d},
    },
    prelude::*,
};

const BALL_SIZE: f32 = 10.;
const CANVAS_SIZE: Vec2 = Vec2::new(1280., 720.);
const BRICK_SIZE: Vec2 = Vec2::new(80., 40.);
const DEFAULT_PADDLE_SIZE: Vec2 = Vec2::new(200., 20.);
const PADDLE_SPEED: f32 = 600.;
const BALL_SPEED: f32 = 100.;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Wall(Plane2d);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct HalfSize(Vec2);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::from(SKY_300)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, (paddle_controls, ball_movement))
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
        Velocity(Vec2::new(0.0, -BALL_SPEED)),
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::from(SLATE_900))),
        Transform::from_xyz(0.0, -50.0, 0.0),
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
        Transform::from_xyz(0.0, -CANVAS_SIZE.y / 2.0, 0.0),
    ));
    // Top wall
    commands.spawn((
        Wall(Plane2d::new(Vec2::NEG_Y)),
        Transform::from_xyz(0.0, CANVAS_SIZE.y / 2.0, 0.0),
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(DEFAULT_PADDLE_SIZE),
            color: Color::from(SKY_50),
            ..default()
        },
        Transform::from_xyz(0.0, -CANVAS_SIZE.y * 3.0 / 8.0, 0.0),
        Paddle,
        HalfSize(DEFAULT_PADDLE_SIZE / 2.),
    ));

    let n_rows: i32 = 6;
    let n_columns: i32 = 13;
    let base_color = Oklcha::from(SKY_300);

    for row in 0..n_rows {
        for column in 0..n_columns {
            commands.spawn((
                Brick,
                Sprite {
                    custom_size: Some(BRICK_SIZE),
                    color: base_color.into(),
                    ..default()
                },
                Transform::from_xyz(
                    -480. + BRICK_SIZE.x * column as f32,
                    240. - BRICK_SIZE.y * row as f32,
                    0.0,
                ),
                HalfSize(BRICK_SIZE / 2.),
            ));
        }
    }
}

fn ball_movement(
    mut query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    walls: Query<(&Wall, &Transform), Without<Ball>>,
    aabb_colliders: Query<(Entity, &Transform, &HalfSize), Without<Ball>>,
    paddles: Query<(), With<Paddle>>,
    bricks: Query<(), With<Brick>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut velocity, mut transform) in &mut query {
        let ball_movement_this_frame = velocity.0 * time.delta_secs();
        let ball_move_distance = ball_movement_this_frame.length();
        let ball_ray = Ray2d::new(transform.translation.xy(), Dir2::new(velocity.0).unwrap());

        for (wall, transform) in &walls {
            if let Some(distance_to_wall) =
                ball_ray.intersect_plane(transform.translation.xy(), wall.0)
                && ball_move_distance >= distance_to_wall
            {
                velocity.0 = velocity.0.reflect(wall.0.normal.as_vec2());
                return;
            }
        }

        let ball_cast = RayCast2d::from_ray(ball_ray, ball_move_distance);

        if let Some((entity, origin, aabb_collider, _)) = aabb_colliders
            .iter()
            .filter_map(|(entity, origin, half_size)| {
                let collider = Aabb2d::new(origin.translation.xy(), half_size.0);

                let distance = ball_cast.aabb_intersection_at(&collider)?;

                Some((entity, origin, collider, distance))
            })
            .min_by_key(|(_, _, _, distance)| FloatOrd(*distance))
        {
            if paddles.get(entity).is_ok() {
                let direction_vector = transform.translation.xy() - origin.translation.xy();
                let angle = direction_vector.to_angle();
                let linear_angle = angle.clamp(0., PI) / PI;
                let softened_angle = FRAC_PI_4.lerp(PI - FRAC_PI_4, linear_angle);
                velocity.0 = Vec2::from_angle(softened_angle) * velocity.0.length();
            } else if bricks.get(entity).is_ok() {
                let (hit_normal, _) = [
                    (
                        Vec2::new(origin.translation.x, aabb_collider.max.y),
                        Plane2d::new(Vec2::Y),
                    ),
                    (
                        Vec2::new(origin.translation.x, aabb_collider.min.y),
                        Plane2d::new(Vec2::NEG_Y),
                    ),
                    (
                        Vec2::new(aabb_collider.max.x, origin.translation.y),
                        Plane2d::new(Vec2::X),
                    ),
                    (
                        Vec2::new(aabb_collider.min.x, origin.translation.y),
                        Plane2d::new(Vec2::NEG_X),
                    ),
                ]
                .into_iter()
                .filter_map(|(location, plane)| {
                    ball_ray
                        .intersect_plane(location, plane)
                        .map(|hit_distance| (plane.normal, hit_distance))
                })
                .min_by_key(|(_, hit_distance)| FloatOrd(*hit_distance))
                .unwrap();
                commands.entity(entity).despawn();
                velocity.0 = velocity.0.reflect(hit_normal.into());
            }
            break;
        }

        transform.translation += ball_movement_this_frame.extend(0.0);
    }
}

fn paddle_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut paddles: Query<(&mut Transform, &HalfSize), With<Paddle>>,
    time: Res<Time>,
) {
    for (mut transform, half_size) in &mut paddles {
        if keys.pressed(KeyCode::KeyA)
            && transform.translation.x - half_size.0.x > -CANVAS_SIZE.x / 2.
        {
            println!("{}", transform.translation.x);
            transform.translation.x -= PADDLE_SPEED * time.delta_secs();
        } else if keys.pressed(KeyCode::KeyD)
            && transform.translation.x + half_size.0.x < CANVAS_SIZE.x / 2.
        {
            transform.translation.x += PADDLE_SPEED * time.delta_secs();
        }
    }
}
