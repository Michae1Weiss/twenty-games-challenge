use bevy::{camera::ScalingMode, prelude::*};

pub const CANVAS_SIZE: Vec2 = Vec2::new(1280., 720.);
const CANVAS_PADDING: Vec2 = Vec2::new(80., 40.);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: CANVAS_SIZE.x + CANVAS_PADDING.x,
                min_height: CANVAS_SIZE.y + CANVAS_PADDING.y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
