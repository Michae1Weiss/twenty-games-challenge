use std::f32::consts::{FRAC_PI_6, PI};

use bevy::{
    color::palettes::{
        css::{BLACK, WHITE, WHITE_SMOKE},
        tailwind::{SKY_600, SLATE_900},
    },
    math::{
        FloatOrd,
        bounding::{Aabb2d, BoundingCircle, IntersectsVolume, RayCast2d},
    },
    prelude::*,
    sprite::Anchor,
};
use bevy_hanabi::prelude::*;

use crate::{
    CANVAS_SIZE, GameState,
    audio::PlaySfx,
    game::{
        Brick, DEFAULT_PADDLE_SIZE, HalfSize, Paddle, PaddleMovement, Velocity, Wall,
        assets::TextureAssets,
    },
};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (ball_movement, on_ball_intersects_respawn_area).run_if(in_state(GameState::Playing)),
    )
    .add_systems(OnEnter(GameState::Playing), spawn_ball);
}

const BALL_SIZE: f32 = 10.;
const BALL_SPIN_MAGNITUDE: f32 = 0.005; // radian/sec
const RIBBON_SPAWN_RATE: f32 = 64.;
const RIBBON_LIFETIME: f32 = 1.5; // Seconds
const RIBBON_PARTICLE_CAPACITY: u32 = 100; // 64 * 1.5 = 98 or 100 (rounded)

#[derive(Component)]
struct Ball;

/// Component representing a ball that curves when hit
#[derive(Component)]
struct Spin {
    /// direction & magnitude of curve force
    curve_force: f32,
}

#[derive(Component)]
struct RespawnBallArea;

fn build_ribbon_effect() -> EffectAsset {
    let writer = ExprWriter::new();

    let init_pos_attr =
        SetAttributeModifier::new(Attribute::POSITION, writer.lit(Vec3::ZERO).expr());
    let init_age_attr = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.0).expr());
    let init_lifetime_attr =
        SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(RIBBON_LIFETIME).expr());
    let init_size_attr = SetAttributeModifier::new(Attribute::SIZE, writer.lit(0.5).expr());
    let init_ribbon_id = SetAttributeModifier::new(Attribute::RIBBON_ID, writer.lit(0u32).expr());

    let gradient =
        bevy_hanabi::Gradient::linear(Vec4::new(1.0, 1., 1., 1.), Vec4::new(1.0, 1., 1., 0.));

    let color_over_time_modifier = ColorOverLifetimeModifier::new(gradient);

    let size_over_time_modifier = SizeOverLifetimeModifier {
        gradient: bevy_hanabi::Gradient::linear(Vec3::splat(10.0), Vec3::ZERO),
        ..default()
    };
    let spawner = SpawnerSettings::rate(RIBBON_SPAWN_RATE.into());

    EffectAsset::new(RIBBON_PARTICLE_CAPACITY, spawner, writer.finish())
        .with_motion_integration(bevy_hanabi::MotionIntegration::None)
        .with_simulation_space(bevy_hanabi::SimulationSpace::Global)
        .init(init_pos_attr)
        .init(init_age_attr)
        .init(init_lifetime_attr)
        .init(init_size_attr)
        .init(init_ribbon_id)
        .render(color_over_time_modifier)
        .render(size_over_time_modifier)
}

fn spawn_ball(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    texture_assets: Res<TextureAssets>,
) {
    commands.spawn((
        Sprite {
            image: texture_assets.danger_zone.clone(),
            custom_size: Some(Vec2::new(
                CANVAS_SIZE.x,
                CANVAS_SIZE.y / 8.0 - DEFAULT_PADDLE_SIZE.y / 2.0,
            )),
            // color: .into(),
            ..default()
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(0.0, -CANVAS_SIZE.y / 2., -1.0),
        RespawnBallArea,
        DespawnOnExit(GameState::Playing),
    ));

    info!(
        "x: {}, y:{}",
        CANVAS_SIZE.x,
        CANVAS_SIZE.y / 8.0 - DEFAULT_PADDLE_SIZE.y / 2.0
    );

    let effect = build_ribbon_effect();
    let effect = effects.add(effect);

    commands.spawn((
        Ball,
        Spin { curve_force: 0.0 },
        ParticleEffect::new(effect),
        Velocity(Vec2::new(-20., -480.)),
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::from(SLATE_900))),
        Transform::from_xyz(0.0, -50.0, 0.0),
        DespawnOnExit(GameState::Playing),
        children![(
            Mesh2d(meshes.add(Circle::new(BALL_SIZE - 1.0))),
            MeshMaterial2d(materials.add(Color::from(WHITE))),
            Transform::from_xyz(0.0, 0.0, 1.0)
        )],
    ));
}

fn ball_movement(
    mut balls: Query<(&mut Velocity, &mut Transform, &mut Spin), With<Ball>>,
    walls: Query<(&Wall, &Transform), Without<Ball>>,
    aabb_colliders: Query<(Entity, &Transform, &HalfSize), Without<Ball>>,
    paddles: Query<&PaddleMovement, With<Paddle>>,
    bricks: Query<(), With<Brick>>,
    time: Res<Time>,
    mut commands: Commands,
    mut play_sfx_writer: MessageWriter<PlaySfx>,
) {
    for (mut velocity, mut transform, mut spin) in &mut balls {
        velocity.0 = Vec2::from_angle(spin.curve_force).rotate(velocity.0);
        let ball_movement_this_frame = velocity.0 * time.delta_secs();
        let ball_move_distance = ball_movement_this_frame.length();
        let ball_ray = Ray2d::new(transform.translation.xy(), Dir2::new(velocity.0).unwrap());

        for (wall, transform) in &walls {
            if let Some(distance_to_wall) =
                ball_ray.intersect_plane(transform.translation.xy(), wall.0)
                && ball_move_distance >= distance_to_wall
            {
                velocity.0 = velocity.0.reflect(wall.0.normal.as_vec2());

                play_sfx_writer.write(PlaySfx::BallWall);
                spin.curve_force = 0.0;
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
            if let Some(paddle_movement) = paddles.get(entity).ok() {
                let direction_vector = transform.translation.xy() - origin.translation.xy();
                let angle = direction_vector.to_angle();
                let linear_angle = angle.clamp(0., PI) / PI;
                let softened_angle = FRAC_PI_6.lerp(PI - FRAC_PI_6, linear_angle);
                velocity.0 = Vec2::from_angle(softened_angle) * velocity.0.length();
                play_sfx_writer.write(PlaySfx::BallPaddle);
                spin.curve_force = match paddle_movement {
                    PaddleMovement::Left => -BALL_SPIN_MAGNITUDE,
                    PaddleMovement::Right => BALL_SPIN_MAGNITUDE,
                    PaddleMovement::Idle => 0.0,
                };

                info!("Paddle movement: {paddle_movement:?}");
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
                play_sfx_writer.write(PlaySfx::BrickBreak);
                commands.entity(entity).despawn();
                if spin.curve_force == 0.0 {
                    velocity.0 = velocity.0.reflect(hit_normal.into());
                }
            }
            break;
        }

        transform.translation += ball_movement_this_frame.extend(0.0);
    }
}

fn on_ball_intersects_respawn_area(
    respawn_area: Single<(&Transform, &Sprite), With<RespawnBallArea>>,
    balls: Query<&Transform, With<Ball>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for &ball in &balls {
        let circle = BoundingCircle::new(ball.translation.xy(), BALL_SIZE);
        if Aabb2d::new(
            respawn_area.0.translation.xy(),
            respawn_area.1.custom_size.unwrap() / Vec2::splat(2.),
        )
        .intersects(&circle)
        {
            next_state.set(GameState::GameOver);
        }
    }
}
