use bevy::{color::palettes::css::ORANGE, prelude::*};

use crate::{
    game::{
        Velocity,
        ball::Ball,
        collision::{
            Collider, CollisionResponse, SpinEffect, TrajectoryParams, predict_trajectory,
        },
        paddle::Paddle,
        phase::GamePhase,
    },
    state::Screen,
};

const DOT_SPACING: f32 = 32.0; // world units between dots
const DOT_RADIUS: f32 = 3.0;
const DOT_Z: f32 = 5.0; // above the playfield; tune to your layers
const MAX_LENGTH: f32 = 1200.0; // total arc-length budget for the whole line
const MAX_BOUNCES: u32 = 8; // belt-and-suspenders cap on reflections
const DOT_POOL: usize = 80; // ≈ MAX_LENGTH / DOT_SPACING + slack

#[derive(Component)]
struct AimDot;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GamePhase::Sniper), spawn_dot_pool)
        .add_systems(OnExit(GamePhase::Sniper), despawn_dot_pool)
        .add_systems(
            Update,
            render_aim_line.run_if(in_state(GamePhase::Sniper).and(in_state(Screen::None))),
        );
}

fn spawn_dot_pool(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(DOT_RADIUS));
    let material = materials.add(Color::from(ORANGE));

    for _ in 0..DOT_POOL {
        commands.spawn((
            AimDot,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(0.0, 0.0, DOT_Z),
            Visibility::Hidden,
        ));
    }
}

fn despawn_dot_pool(mut commands: Commands, dots: Query<Entity, With<AimDot>>) {
    for dot in &dots {
        commands.entity(dot).despawn();
    }
}

fn render_aim_line(
    balls: Query<(&Transform, &Velocity), With<Ball>>,
    paddles: Query<Entity, With<Paddle>>,
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
    // Without<Collider>/Without<Ball> keep this disjoint from the queries above,
    // so Bevy can prove the mutable Transform access is conflict-free.
    mut dots: Query<
        (&mut Transform, &mut Visibility),
        (With<AimDot>, Without<Ball>, Without<Collider>),
    >,
    mut path: Local<Vec<Vec2>>,
) {
    // Endgame normally has one ball. For multiball, pick the descending ball
    // nearest the paddle instead of bailing here.
    let (Ok((ball_transform, velocity)), Ok(paddle)) = (balls.single(), paddles.single()) else {
        hide_all(dots.iter_mut());
        return;
    };

    let Ok(direction) = Dir2::new(velocity.0) else {
        hide_all(dots.iter_mut());
        return;
    };

    let params = TrajectoryParams {
        max_length: MAX_LENGTH,
        max_bounces: MAX_BOUNCES,
    };
    let first_hit = predict_trajectory(
        ball_transform.translation.xy(),
        direction,
        velocity.0.length(),
        &colliders,
        &params,
        &mut path,
    );

    // Only draw when the shot actually leads into the paddle — that's the
    // bounce the player aims with. (Drop this check to draw toward any target.)
    if first_hit != Some(paddle) {
        hide_all(dots.iter_mut());
        return;
    }

    place_dots(&path, dots.iter_mut());
}

/// Walks the polyline at fixed arc-length spacing, placing a dot at each step
/// and carrying the remainder across segment boundaries so spacing stays even
/// through corners. Leftover dots are hidden.
fn place_dots<'a>(
    path: &[Vec2],
    mut pool: impl Iterator<Item = (Mut<'a, Transform>, Mut<'a, Visibility>)>,
) {
    let mut carry = 0.0_f32;
    'walk: for window in path.windows(2) {
        let (a, b) = (window[0], window[1]);
        let length = a.distance(b);
        if length < f32::EPSILON {
            continue;
        }
        let unit = (b - a) / length;
        let mut traveled = carry;
        while traveled <= length {
            let Some((mut transform, mut visibility)) = pool.next() else {
                break 'walk;
            };
            transform.translation = (a + unit * traveled).extend(DOT_Z);
            *visibility = Visibility::Visible;
            traveled += DOT_SPACING;
        }
        carry = traveled - length;
    }
    hide_all(pool);
}

fn hide_all<'a>(pool: impl Iterator<Item = (Mut<'a, Transform>, Mut<'a, Visibility>)>) {
    for (_, mut visibility) in pool {
        *visibility = Visibility::Hidden;
    }
}
