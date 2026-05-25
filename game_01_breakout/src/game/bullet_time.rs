use crate::{
    game::{Velocity, ball::Ball, brick::Brick, paddle::Paddle},
    state::{GameState, Screen},
};
use bevy::{math::curve::EaseFunction, prelude::*};

/// Game-feel tuning. A Reflect resource so you can dial it live in the
/// inspector instead of recompiling — exactly the constants-as-data case.
#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct BulletTime {
    /// Only active once fewer than this many bricks remain.
    pub brick_threshold: u32,
    /// Vertical gap (ball above paddle) at which slowdown begins.
    pub near_distance: f32,
    /// Slowest the world goes (0.2 = 20% speed).
    pub min_scale: f32,
    /// How fast the scale eases toward its target, per REAL second.
    pub ramp_rate: f32,
}

impl Default for BulletTime {
    fn default() -> Self {
        Self {
            brick_threshold: 5,
            near_distance: 400.0,
            min_scale: 0.2,
            ramp_rate: 6.0,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<BulletTime>()
        .register_type::<BulletTime>()
        // Runs only while actually playing (not paused, not in a menu).
        .add_systems(
            Update,
            drive_bullet_time.run_if(in_state(GameState::Playing).and(in_state(Screen::None))),
        )
        // Make sure virtual time is back to normal whenever we stop playing.
        .add_systems(OnExit(GameState::Playing), reset_time_scale);
}

fn target_scale(
    bullet_time: &BulletTime,
    brick_count: u32,
    balls: impl Iterator<Item = (Vec2, Vec2)>, // (position, velocity)
    paddle_y: f32,
) -> f32 {
    if brick_count >= bullet_time.brick_threshold {
        return 1.0; // not the endgame — full speed
    }
    let mut slowest = 1.0_f32;
    for (position, velocity) in balls {
        if velocity.y >= 0.0 {
            continue; // moving up / away from paddle — don't slow
        }
        let gap = position.y - paddle_y;
        if gap < 0.0 || gap > bullet_time.near_distance {
            continue; // below paddle (lost) or too far to matter
        }
        // 0 at the paddle, 1 at the edge of the near zone.
        let t = gap / bullet_time.near_distance;
        // SmoothStep gives the Matrix-y ease: gentle at the ends, steep in the
        // middle. sample() returns Option (out-of-domain) — t is already 0..1.
        let eased = EaseFunction::SmoothStep.sample(t).unwrap_or(t);
        // gap→0 ⇒ eased→0 ⇒ min_scale ;  gap→near ⇒ eased→1 ⇒ 1.0
        let scale = bullet_time.min_scale + (1.0 - bullet_time.min_scale) * eased;
        slowest = slowest.min(scale);
    }
    slowest
}

fn drive_bullet_time(
    real_time: Res<Time<Real>>,
    bullet_time: Res<BulletTime>,
    bricks: Query<(), With<Brick>>,
    balls: Query<(&Transform, &Velocity), With<Ball>>,
    paddle: Query<&Transform, With<Paddle>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    let Ok(paddle_transform) = paddle.single() else {
        return;
    };

    let target = target_scale(
        &bullet_time,
        bricks.iter().count() as u32,
        balls
            .iter()
            .map(|(transform, velocity)| (transform.translation.truncate(), velocity.0)),
        paddle_transform.translation.y,
    );

    // Frame-rate-independent exponential smoothing — and crucially driven by
    // REAL delta. If this used virtual delta, slowing time would also slow the
    // ease-OUT, and recovery would crawl. The controller of the clock must not
    // live on the clock it controls.
    let current = virtual_time.relative_speed();
    let alpha = 1.0 - (-bullet_time.ramp_rate * real_time.delta_secs()).exp();
    let next = current + (target - current) * alpha;
    virtual_time.set_relative_speed(next.clamp(bullet_time.min_scale, 1.0));
}

fn reset_time_scale(mut virtual_time: ResMut<Time<Virtual>>) {
    virtual_time.set_relative_speed(1.0);
}
