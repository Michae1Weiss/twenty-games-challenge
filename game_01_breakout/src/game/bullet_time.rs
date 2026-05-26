use crate::{
    game::{Velocity, ball::Ball, paddle::Paddle, phase::GamePhase},
    state::{GameState, Screen},
};
use bevy::{math::curve::EaseFunction, prelude::*};

/// Game-feel tuning, live-editable in the inspector.
#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct BulletTime {
    /// Vertical gap (ball above paddle) at which slowdown begins.
    pub near_distance: f32,
    /// Slowest the world goes (0.1 = 10% speed).
    pub min_scale: f32,
    /// How fast the scale eases toward its target, per REAL second.
    pub ramp_rate: f32,
}

impl Default for BulletTime {
    fn default() -> Self {
        Self {
            near_distance: 200.0,
            min_scale: 0.1,
            ramp_rate: 24.0,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<BulletTime>()
        .register_type::<BulletTime>()
        // Sniper is a substate of Playing, so this implies in-game already.
        .add_systems(
            Update,
            drive_bullet_time.run_if(in_state(GamePhase::Sniper).and(in_state(Screen::None))),
        )
        // Restore normal time whenever we leave sniper OR stop playing.
        .add_systems(OnExit(GamePhase::Sniper), reset_time_scale)
        .add_systems(OnExit(GameState::Playing), reset_time_scale);
}

fn target_scale(
    bullet_time: &BulletTime,
    balls: impl Iterator<Item = (Vec2, Vec2)>, // (position, velocity)
    paddle_y: f32,
) -> f32 {
    let mut slowest = 1.0_f32;
    for (position, velocity) in balls {
        if velocity.y >= 0.0 {
            continue; // moving away from paddle — don't slow
        }
        let gap = position.y - paddle_y;
        if gap < 0.0 || gap > bullet_time.near_distance {
            continue; // below paddle (lost) or too far to matter
        }
        let t = gap / bullet_time.near_distance; // 0 at paddle, 1 at edge of zone
        let eased = EaseFunction::SmoothStep.sample(t).unwrap_or(t);
        let scale = bullet_time.min_scale + (1.0 - bullet_time.min_scale) * eased;
        slowest = slowest.min(scale);
    }
    slowest
}

fn drive_bullet_time(
    real_time: Res<Time<Real>>,
    bullet_time: Res<BulletTime>,
    balls: Query<(&Transform, &Velocity), With<Ball>>,
    paddle: Query<&Transform, With<Paddle>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    let Ok(paddle_transform) = paddle.single() else {
        return;
    };

    let target = target_scale(
        &bullet_time,
        balls
            .iter()
            .map(|(transform, velocity)| (transform.translation.truncate(), velocity.0)),
        paddle_transform.translation.y,
    );

    // Frame-rate-independent exponential smoothing, driven by REAL delta so the
    // ease-out doesn't slow down along with the world it controls.
    let current = virtual_time.relative_speed();
    let alpha = 1.0 - (-bullet_time.ramp_rate * real_time.delta_secs()).exp();
    let next = current + (target - current) * alpha;
    virtual_time.set_relative_speed(next.clamp(bullet_time.min_scale, 1.0));
}

fn reset_time_scale(mut virtual_time: ResMut<Time<Virtual>>) {
    virtual_time.set_relative_speed(1.0);
}
