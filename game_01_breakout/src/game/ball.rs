use bevy::{
    color::palettes::{css::WHITE, tailwind::SLATE_900},
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};
use bevy_hanabi::prelude::*;
use bevy_transform_interpolation::prelude::TransformInterpolation;

use crate::{
    GameState,
    game::{
        EndRound, Velocity,
        collision::{Collider, Collision, CollisionResponse, SpinEffect, deflect, first_contact},
        respawn::RespawnBallArea,
    },
};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (simulate_balls, on_ball_intersects_respawn_area).run_if(in_state(GameState::Playing)),
    );
}

const BALL_SIZE: f32 = 10.;
const RIBBON_SPAWN_RATE: f32 = 64.;
const RIBBON_LIFETIME: f32 = 1.5; // Seconds
const RIBBON_PARTICLE_CAPACITY: u32 = 100; // 64 * 1.5 = 98 or 100 (rounded)

#[derive(Component)]
pub struct Ball;

/// Component representing a ball that curves when hit
#[derive(Component)]
pub struct Spin {
    /// direction & magnitude of curve force
    curve_force: f32,
}

pub struct SpawnBall<M: Bundle> {
    position: Vec2,
    velocity: Vec2,
    marker: M,
}

impl<M: Bundle> SpawnBall<M> {
    pub fn new(position: Vec2, velocity: Vec2, marker: M) -> Self {
        Self {
            position,
            velocity,
            marker,
        }
    }
}

impl<M: Bundle> Command for SpawnBall<M> {
    fn apply(self, world: &mut World) -> () {
        let ball_ribbon_effect =
            world.resource_scope(|_, mut effect_assets: Mut<Assets<EffectAsset>>| {
                let ribbon_effect = build_ribbon_effect();
                effect_assets.add(ribbon_effect)
            });

        let ball_outer_mesh = world
            .resource_scope(|_, mut meshes: Mut<Assets<Mesh>>| meshes.add(Circle::new(BALL_SIZE)));

        let ball_outer_material =
            world.resource_scope(|_, mut materials: Mut<Assets<ColorMaterial>>| {
                materials.add(Color::from(SLATE_900))
            });

        let ball_inner_mesh = world.resource_scope(|_, mut meshes: Mut<Assets<Mesh>>| {
            meshes.add(Circle::new(BALL_SIZE - 1.0))
        });

        let ball_inner_material =
            world.resource_scope(|_, mut materials: Mut<Assets<ColorMaterial>>| {
                materials.add(Color::from(WHITE))
            });

        world.spawn((
            Ball,
            Spin { curve_force: 0.0 },
            ParticleEffect::new(ball_ribbon_effect),
            Velocity(self.velocity),
            Mesh2d(ball_outer_mesh),
            MeshMaterial2d(ball_outer_material),
            Transform::from_xyz(self.position.x, self.position.y, 0.0),
            TransformInterpolation,
            self.marker,
            children![(
                Mesh2d(ball_inner_mesh),
                MeshMaterial2d(ball_inner_material),
                Transform::from_xyz(0.0, 0.0, 1.0)
            )],
        ));
    }
}

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

pub fn simulate_balls(
    mut balls: Query<(Entity, &mut Velocity, &mut Transform, &mut Spin), With<Ball>>,
    colliders: Query<
        (
            Entity,
            &Transform,
            &Collider,
            &CollisionResponse,
            Option<&SpinEffect>,
        ),
        Without<Ball>,
    >,
    time: Res<Time>,
    mut collisions: MessageWriter<Collision>,
) {
    for (ball, mut velocity, mut transform, mut spin) in &mut balls {
        velocity.0 = Vec2::from_angle(spin.curve_force).rotate(velocity.0);
        let step = velocity.0 * time.delta_secs();
        let ray = Ray2d::new(transform.translation.xy(), Dir2::new(velocity.0).unwrap());

        if let Some((contact, response, spin_effect)) =
            first_contact(ray, step.length(), &colliders)
        {
            match response {
                CollisionResponse::Reflect => {
                    velocity.0 = velocity.0.reflect(contact.contact_surface_normal.as_vec2())
                }
                CollisionResponse::Deflect => {
                    velocity.0 = deflect(
                        transform.translation.xy(),
                        contact.hit_body_center,
                        velocity.0.length(),
                    )
                }
                CollisionResponse::ReflectOrPierce if spin.curve_force == 0.0 => {
                    velocity.0 = velocity.0.reflect(contact.contact_surface_normal.as_vec2())
                }
                CollisionResponse::ReflectOrPierce => {} // curving: punch through
            }

            match spin_effect {
                Some(SpinEffect::Clear) => spin.curve_force = 0.0,
                Some(SpinEffect::Impart(force)) => spin.curve_force = force,
                None => {} // brick: leave the curve untouched
            }

            collisions.write(Collision {
                ball,
                hit: contact.entity,
                point: contact.contact_point,
                normal: contact.contact_surface_normal,
            });
            continue;
        }

        transform.translation += step.extend(0.0);
    }
}

fn on_ball_intersects_respawn_area(
    respawn_area: Single<(&Transform, &Sprite), With<RespawnBallArea>>,
    balls: Query<&Transform, With<Ball>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut end_round_writer: MessageWriter<EndRound>,
) {
    for &ball in &balls {
        let circle = BoundingCircle::new(ball.translation.xy(), BALL_SIZE);
        if Aabb2d::new(
            respawn_area.0.translation.xy(),
            respawn_area.1.custom_size.unwrap() / Vec2::splat(2.),
        )
        .intersects(&circle)
        {
            end_round_writer.write_default();
            next_state.set(GameState::GameOver);
        }
    }
}
